
fn sum(nums:Vec<i32>) ->i32{
    let mut sum = 0;
    for num in nums{
        sum = sum+num;
}
return sum;
}

fn avg(nums: Vec<i32>) ->i64{
    let mut sum = 0;
    let count = nums.len();
    for num in nums{
        sum = sum+num;
    }
    sum as i64/ count as i64
}


fn doubledArr(nums: Vec<i32>){
    let mut newArr: Vec<i32> = Vec::new();
    for mut num in nums{
        num = num*2;
        newArr.push(num);
    }
    for i in newArr{
        println!("{}", i);
    }
}

fn biggestValue(nums: Vec<i32>) ->i32{
    let mut max = nums[0];
    for i in 0..nums.len(){
        if nums[i]>max{
            max = nums[i];
        }
    }
    return max;
}

fn onlyEven(nums: Vec<i32>) ->Vec<i32>{
    let mut newArr: Vec<i32> = Vec::new();
    for num in nums{
        if num %2 ==0{
           newArr.push(num);
        }
    }
    return newArr;
}

fn allPos(nums:Vec<i32>) ->bool{
    let mut count = nums.len();
    for num in nums{
        if num >0{
            count = count-1;
        }
    }
    if count>0{
        return false;
    }else{
        return true;
    }
}

fn reverString(names: Vec<String>) ->Vec<String>{
    let mut newStr: Vec<String> = Vec::new();

    let mut i = names.len();
    while i>0{
        i = i-1;
        newStr.push(names[i].clone());
    }
    return newStr;
}


fn main() {

    // let mut numbers = vec! [10, -79, 77, 123, 50];
    // numbers.push(60);
    // numbers.push(87);

    let mut names: Vec<String> = vec!["Sarah".to_string(), "Michael".to_string(), "David".to_string(), "Batia".to_string()];

    let newNames = reverString(names);
    for name in newNames{
        println!("{}", name);
    }

    // for num in numbers{
    //     println!("{}", num);
    // }
// println!("{}", numbers.len());
    
// println!("{}", sum(numbers));

// println!("{}", avg(numbers));
// doubledArr(numbers);
// println!("{}", biggestValue(numbers));

// let mut arr = onlyEven(numbers);
// for num in arr{
//     println!("{}", num);
// }
// println!("{}", allPos(numbers));
}
