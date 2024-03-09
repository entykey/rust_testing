// upcoder: LOPPS

#[allow(dead_code)]
fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while a * b != 0 {
        if a > b {
            a %= b;
        } else {
            b %= a;
        }
    }
    a + b
}

struct PS {
    tu: i32,    // may overflow if it's big num
    mau: i32,
}

impl std::fmt::Display for PS {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // return 1 when it's 1/1
        if self.mau == 1 {
            write!(f, "{}", self.tu)
        } else {
            write!(f, "{}/{}", self.tu, self.mau)
        }
    }
}

#[allow(dead_code)]
impl PS {
    fn new(tu: i32, mau: i32) -> Self {
        PS { tu, mau }
    }
    
    fn default() -> Self {
        PS { tu: 0, mau: 1 }
    }

    fn nghich_dao(&self) -> Self {  // inverse
        PS::new(self.mau, self.tu)
    }

    // fn gut_gon(&self) -> Self {
    //     let x = gcd(self.tu, self.mau);
    //     PS::new(self.tu / x, self.mau / x)
    // }

    fn gut_gon(&self) -> Self {
        let x = gcd(self.tu, self.mau);
        if x != 1 {
            PS::new(self.tu / x, self.mau / x)
        } else {
            PS::new(self.tu, self.mau)
        }
    }

    fn cong_ps(&self, other: &PS) -> PS {
        PS::new(self.tu * other.mau + self.mau * other.tu, self.mau * other.mau)
    }

    fn tru_ps(&self, other: &PS) -> PS {
        PS::new(self.tu * other.mau - self.mau * other.tu, self.mau * other.mau)
    }

    fn nhan_ps(&self, other: &PS) -> PS {
        PS::new(self.tu * other.tu, self.mau * other.mau)
    }

    fn chia_ps(&self, other: &PS) -> PS {
        PS::new(self.tu * other.mau, self.mau * other.tu)
    }

    fn xuat(&self) {
        println!("{}", self)    // PS has std::fmt::Display trait implemented
    }

}


fn main() {
    
    // reading input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let p1 = PS::new(nums[0], nums[1]);

    // reading input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();


    let calc_time: std::time::Instant = std::time::Instant::now();

    let k: i32 = input.trim().parse().unwrap();

    println!("{}", p1.tu);
    println!("{}", p1.mau);

    let p2 = p1.nghich_dao();
    print!("p2 = p1 reversed: ");
    p2.xuat();

    let p3 = p1.gut_gon();
    print!("p3 = p1 simplified: ");
    p3.xuat();

    let p4 = PS::new(p1.tu + k, p1.mau + k);
    print!("p4 = 2+k/4+k: ");
    p4.xuat();

    let p5 = p4.nhan_ps(&p1).gut_gon();
    print!("p5 = p4 * p1, simplified: ");
    p5.xuat();

    let p6 = p1.cong_ps(&p3).gut_gon();
    print!("p6 = p1 + p3, simplified: ");
    p6.xuat();


    // end of main
    let duration: std::time::Duration = calc_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);

}

/*
Input (2 lines):
2 4
1

Ouput:
2
4
4/2
1/2
3/5
3/10
*/



/* 
C++ original:

/* ly thuyet 2 */

#include<iostream>
#include <algorithm> // for __lcm & __gcd
using namespace std;

// we will use __gcd() instead of this!!
int UCLN(int a, int b){
	while( a*b != 0){
		if(a>b) a%=b;
		else b%=a;
	}
	return a+b;
}

class PS {
	int tu, mau;
public:
	// get & set
	int getTu() {return tu;}
	void setTu(int x) {tu = x;}
	int getMau() {return mau;}
	void setMau(int x) {mau = x;}
	//ktao - huy
	PS(int _tu=0, int _mau=1) {tu=_tu; mau=_mau;}
	PS(const PS &p) { tu = p.tu; mau = p.mau;}
	~PS(){}
	// nhap - xuat
	void Nhap() {cin>>tu>>mau;}
	void Xuat() {cout<<tu<<"/"<<mau<<"\n";}
	// ham bo tro
	//void NghichDao() { int tmp = tu; tu = mau; mau = tmp;} // cach 1, code main khac !
	PS NghichDao() {return PS(mau, tu);}
	PS GutGon() {
		int x = __gcd(tu, mau); // algorithm lib required
		//tu /= x;
		//mau /= x;
		PS p1;
		p1.tu = this->tu/x;
		p1.mau = this->mau/x;
		return p1;
	}
	PS CongPS(PS p1){ // this + p1
		PS p = PS(tu*p1.mau + mau*p1.tu, mau*p1.mau);
		return p;
	}
	PS TruPS(PS p1){
		PS p = PS(tu*p1.mau - mau*p1.tu, mau*p1.mau);
		return p;
	}
	PS operator +(const PS& p1){
		PS p = PS(tu*p1.mau + mau*p1.tu, mau*p1.mau);
		return p;
	}
	PS operator -(const PS& p1){
		/* PS p = PS(tu*p1.mau - mau*p1.tu, mau*p1.mau);
		return p.GutGon(); */
		PS p;
		p.tu = this->tu*p1.mau - this->mau*p1.tu;
		p.mau = this->mau*p1.mau;
		return p;
	}
	PS operator *(const PS& p1){
		PS p = PS(this->tu*p1.tu, this->mau*p1.mau);
		return p;
	}
	PS operator /(const PS& p1){
		PS p = PS(this->tu*p1.mau, this->mau*p1.tu);
		return p;
	}
};

int main(){
	//PS p1(3,9);
	PS p1;
	int K;
	p1.Nhap();
	cin>>K;
	cout<< p1.getTu() <<endl;
	cout<< p1.getMau() <<endl;
	
	PS p2 = p1.NghichDao();
	//p2 = p2.GutGon();
	p2.Xuat(); // cout<<p2.getTu()<<"/"<<p2.getMau() <<"\n";
	
	PS p3;
	p3 = p1.GutGon();
	p3.Xuat();
	
	PS p4;
	p4.setTu(p1.getTu()+K);
	p4.setMau(p1.getMau()+K);
	p4.Xuat();
	
	PS p5;
	p5 = p4 * p1;
	p5 = p5.GutGon();
	p5.Xuat();
	
	
	return 0;
}
*/