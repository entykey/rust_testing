//Query execution time: 0 ms
//Password verification time: 0 ms
//GetUserRolesAsyncADO & 2 token generation methods execution time: 0 ms
//Remaining code execution time: 16 ms
public async Task<AuthResult> LoginAsyncADOSHA256(LoginModel model, RemoteIpAddress? ip, string? userAgentString)
{
    // Create a stopwatch instance
    Stopwatch stopwatch = new Stopwatch();

    stopwatch.Start(); // Start the stopwatch

    string emailOrUsernameOrPhoneNumber = model.UserNameOrEmailOrPhoneNumber;
    bool isEmail = Regex.IsMatch(emailOrUsernameOrPhoneNumber, @"^[\w-]+(\.[\w-]+)*@([\w-]+\.)+[a-zA-Z]{2,7}$");
    bool isPhoneNumber = Regex.IsMatch(emailOrUsernameOrPhoneNumber, @"^\d+$");

    string query = "SELECT Id, UserName, FullName, Email, PasswordHash, EmailConfirmed FROM Users WHERE ";

    if (isEmail)
    {
        query += "Email = @UserName";
    }
    else if (isPhoneNumber)
    {
        query += "PhoneNumber = @UserName";
    }
    else
    {
        query += "UserName = @UserName";
    }

    string connectionString = _configuration.GetConnectionString("IdentityConnection");

    using (SqlConnection connection = new SqlConnection(connectionString))
    {
        await connection.OpenAsync();

        using (SqlCommand command = new SqlCommand(query, connection))
        {
            command.Parameters.AddWithValue("@UserName", model.UserNameOrEmailOrPhoneNumber);

            using (SqlDataReader reader = await command.ExecuteReaderAsync())
            {
                if (await reader.ReadAsync())
                {
                    stopwatch.Stop(); // Stop the stopwatch
                    Console.WriteLine($"Query execution time: {stopwatch.ElapsedMilliseconds} ms");


                    string userId = reader.GetString(0);
                    string userName = reader.GetString(1);
                    string fullName = reader.GetString(2);
                    string email = reader.GetString(3);
                    string passwordHash = reader.GetString(4);
                    bool emailConfirmed = reader.GetBoolean(5);

                    var applicationUser = new ApplicationUser
                    {
                        Id = userId,
                        UserName = userName,
                        FullName = fullName,
                        Email = email,
                        PasswordHash = passwordHash
                    };

                    stopwatch.Restart(); // Restart the stopwatch

                    if (!await _userManager.CheckPasswordAsync(applicationUser, model.Password))
                    {
                        // Password verification failed
                        List<object> errors = new List<object>
                                {
                                    new
                                    {
                                        code = "InvalidCredentials",
                                        description = "Incorrect email or password!"
                                    }
                                };

                        return new AuthResult
                        {
                            Errors = errors.ToArray()
                        };
                    }

                    stopwatch.Stop(); // Stop the stopwatch
                    Console.WriteLine($"Password verification time: {stopwatch.ElapsedMilliseconds} ms");
                    stopwatch.Restart(); // Restart the stopwatch

                    // Success Login
                    var rolesTask = GetUserRolesAsyncADO(userId);
                    var accessTokenTask = _authTokenService.GenerateAccessTokenAsync(applicationUser, await rolesTask);
                    var refreshTokenTask = _authTokenService.GenerateRefreshTokenAsync();

                    await Task.WhenAll(rolesTask, accessTokenTask, refreshTokenTask);

                    var roles = await rolesTask;
                    var accessToken = await accessTokenTask;
                    var refreshToken = await refreshTokenTask;

                    stopwatch.Stop(); // Stop the stopwatch
                    Console.WriteLine($"GetUserRolesAsyncADO & 2 token generation methods execution time: {stopwatch.ElapsedMilliseconds} ms");


                    stopwatch.Restart();
                    // Retrieve token expiry information
                    var accessTokenValidTo = accessToken.ValidTo;
                    var accessTokenExpiration = DateTime.UtcNow.AddHours(1);
                    var accessTokenExpiresIn = accessTokenExpiration - DateTime.UtcNow;

                    ApplicationUserViewModel userViewModel = new ApplicationUserViewModel
                    {
                        UserName = userName,
                        FullName = fullName,
                        Email = email,
                        Roles = roles.ToList()
                    };

                    #region (Background Job) login history & refreshToken (should use HangFire later)
                    // Enqueue background job to save the refresh token
                    BackgroundJob.Enqueue(() => SaveRefreshToken(userId, refreshToken));

                    // Enqueue background job to store user login history
                    BackgroundJob.Enqueue(() => SaveUserLoginHistory(applicationUser.Id, ip, userAgentString));

                    // Enqueue background job to notify SignalR
                    BackgroundJob.Enqueue(() => NotifySignalR(applicationUser.UserName));

                    #endregion

                    stopwatch.Stop(); // Stop the stopwatch
                    Console.WriteLine($"Remaining code execution time: {stopwatch.ElapsedMilliseconds} ms");

                    return new AuthResult
                    {
                        Succeeded = true,
                        AccessToken = new JwtSecurityTokenHandler().WriteToken(accessToken),
                        RefreshToken = refreshToken,
                        user = userViewModel,
                        Expiration = accessTokenValidTo,
                        ValidFor = $"{(int)accessTokenExpiresIn.TotalHours} hours, {(int)accessTokenExpiresIn.TotalMinutes % 60} minutes"
                    };
                }
            }
            await connection.CloseAsync();
        }
    }

    // User not found
    List<object> notFoundErrors = new List<object>
            {
                new
                {
                    code = "InvalidCredentials",
                    description = "Incorrect email or password!"
                }
            };

    return new AuthResult
    {
        Errors = notFoundErrors.ToArray()
    };
}


private async Task<List<string>> GetUserRolesAsyncADO(string userId)
{
    string query = "SELECT r.Name FROM Roles r JOIN UserRoles ur ON r.Id = ur.RoleId WHERE ur.UserId = @UserId";
    //string connectionString = _configuration.GetConnectionString("IdentityConnection");

    using (SqlConnection connection = new SqlConnection(_connectionString))
    {
        await connection.OpenAsync();
        List<string> roles = new List<string>();

        using (SqlCommand command = new SqlCommand(query, connection))
        {
            command.Parameters.AddWithValue("@UserId", userId);

            using (SqlDataReader reader = await command.ExecuteReaderAsync())
            {
                while (await reader.ReadAsync())
                {
                    string roleName = reader.GetString(0);
                    roles.Add(roleName);
                }
            }
        }
        await connection.CloseAsync();
        return roles;
    }
}


#region Login's Hangfire tasks
public async Task SaveRefreshToken(string userId, string refreshToken)
{
    using (var scope = _serviceProvider.CreateScope())
    {
        var dbContext = scope.ServiceProvider.GetRequiredService<AppDbContext>();

        var user = await dbContext.Users.FindAsync(userId);
        if (user != null)
        {
            user.RefreshToken = refreshToken;
            await dbContext.SaveChangesAsync();
        }
    }
}
public async Task SaveRefreshTokenADO(string userId, string refreshToken)
{
    string query = "UPDATE Users SET RefreshToken = @RefreshToken WHERE Id = @UserId";
    string connectionString = _configuration.GetConnectionString("IdentityConnection");

    using (SqlConnection connection = new SqlConnection(connectionString))
    {
        await connection.OpenAsync();

        using (SqlCommand command = new SqlCommand(query, connection))
        {
            command.Parameters.AddWithValue("@UserId", userId);
            command.Parameters.AddWithValue("@RefreshToken", refreshToken);

            await command.ExecuteNonQueryAsync();
        }
    }
    Console.WriteLine("[AuthService] Hangfire ADO.NET: Saved user refreshToken !!");
}

[AutomaticRetry(Attempts = 0)] // Disable automatic retries for this background job
public async Task SaveUserLoginHistory(string userId, RemoteIpAddress? ip, string? userAgentString)
{
    var uaParser = Parser.GetDefault();
    var clientInfo = uaParser.Parse(userAgentString);

    var userAgentObj = new UserAgentModel
    {
        Name = clientInfo.String,
        OS = new UserAgentOS
        {
            Family = clientInfo.OS.Family,
            Major = clientInfo.OS.Major
        },
        Device = new UserAgentDevice
        {
            Brand = clientInfo.Device.Brand,
            Family = clientInfo.Device.Family,
            Model = clientInfo.Device.Model
        },
        Browser = new UserAgentBrowser
        {
            Family = clientInfo.UA.Family,
            Major = clientInfo.UA.Major
        }
    };

    var loginHistory = new UserLoginHistory
    {
        Id = Guid.NewGuid().ToString(),
        UserId = userId,
        //LoginTime = loginTime,    // the model has taken cared of its default value
        IpAddress = ip,
        //UserAgent = JsonConvert.SerializeObject(userAgentObj)   // for EF
        UserAgent = userAgentObj
    };

    try
    {
        using (var connection = new SqlConnection(_configuration.GetConnectionString("IdentityConnection")))
        {
            await connection.OpenAsync();

            using (var command = new SqlCommand("INSERT INTO UserLoginHistories (Id, UserId, LoginTime, IpAddress, UserAgent) " +
                                                "VALUES (@Id, @UserId, @LoginTime, @IpAddress, @UserAgent)", connection))
            {
                command.Parameters.AddWithValue("@Id", loginHistory.Id);
                command.Parameters.AddWithValue("@UserId", loginHistory.UserId);
                command.Parameters.AddWithValue("@LoginTime", loginHistory.LoginTime);
                command.Parameters.AddWithValue("@IpAddress", JsonConvert.SerializeObject(loginHistory.IpAddress));
                command.Parameters.AddWithValue("@UserAgent", JsonConvert.SerializeObject(loginHistory.UserAgent));

                command.ExecuteNonQuery();
            }
        }

        Console.WriteLine("[AuthService] Hangfire ADO.NET: Saved user login history !!");
    }
    catch (Exception ex)
    {
        Console.WriteLine($"[AuthService] Error while trying to save user login history: {ex.Message}");
    }
}

public void NotifySignalR(string userName)
{
    string room = "123";
    string message = $"{userName} just logged in";

    using (var scope = _serviceProvider.CreateScope())
    {
        var chatHubContext = scope.ServiceProvider.GetRequiredService<IHubContext<ChatServiceHub>>();
        chatHubContext.Clients.All.SendAsync("ReceiveMessage", message);
    }
    Console.WriteLine("[AuthService] Hangfire: Notified user login through signalr !!");
}
#endregion