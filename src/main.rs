fn main() {
    //Defining a static vector
    let vector = vec![1,2,3,4,5,6,7,8,9,10,20,21,33,55,75,87,90];

    //Instance of struct
    let our_struct = FilterCondition{item:5};
    let result = custom_filter(vector, &our_struct);
    
    println!("FILTER RESULT");
    println!("{:?}",result);
}

struct FilterCondition
{
    item:i32,
}
//Method implementation on struct
impl FilterCondition {
    fn is_match(&self,number:&i32) ->bool
    {
        return number % self.item  ==0 ;
    }
}

fn custom_filter(items:Vec<i32>, filter:&FilterCondition) -> Vec<i32>
{
    let result:Vec<i32> =items.into_iter().filter(|x| filter.is_match(x)==true).collect();
    return result;
}