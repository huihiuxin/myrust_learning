
///This is my learning notes
fn main() {
    
    let (a, b):(bool,bool) = (true,false);
    println!("a={},b={}",a,b);
    let a = 10;//不可变变量，但可重影 (let a = 9;)重影意味着生成另一个变量
    let a = a*2;
    let mut b =10;//声明可变变量
    b = b + 35;
    println!("{}",b);
    b = 40;
    println!("{}",b);


    //元组  不同类型
    let tup:(i32,f64,bool) = (500,64.0,true);
    let y = tup;
    println!("{}",y.2);

    //数组 同类型
    let a = [1,2,3,4,5];
    let c: [f32;3] = [1.0,2.0,3.0];
    let d = [3;5];//5个3
    println!("{}",d[4]);
    let a1 = 30;
    let a2 = 31;
    let add_result = add_two_number(a1,a2);
    println!("{}",add_result);
    //函数嵌套
    fn add_two_number(x:i32,y:i32)->i32{
        let m = x+y;
        return m
    }
    //表达式块
    let a3 = {
        let x = 32;
        x+30
    };
    println!("{}",a3);

    //条件语句
    let s = 3;
    let m = if s>0 {2} else if s<0 {-1} else {0};

    //循环语句
    let a = [1,2,3,4,5];
    let mut i = 0;
    //遍历
    println!("while circulation!");
    while i <a.len(){
        println!("{} ",a[i]);
        i += 1;
    }
    println!("for circulation!");
    for j in a.iter() {
        println!("{} ",j);
    }
    println!("subscript circulation!");
    for j in 0..a.len(){
        println!("{} ",a[j]);
    }
    println!("loop circulation!");//无限循环
    i = 0;
    loop{
        let en = a[i];
        if en == 3{
            break;
        }
        println!("{} ",en);
        i += 1;
    }
    //利用loop查找数组元素索引
    let index = ("tyh","fc",12,2.0);
    let index2 = ['A','B','c','D'];
    let mut k = 0;
    let location = loop{
        let et = index2[k];
        if et == 'c'{
            break k;
        }
        k += 1;
    };
    println!("the location is :{} ",location);
    

}
