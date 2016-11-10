#[cfg(test)]
mod tests {
    #[test]
    fn reverse_test() {
        assert_eq!(reverse("hello"),"olleh");
    }
}

//Input:"coderbyte"
//Output:"etybredoc"
fn main(){
 let input = "coderbyte";
 println!("{}", reverse(input).join(""));
}

pub fn reverse(inp: &str) -> Vec<&str> {
   let x:Vec<&str> = inp.rsplit("").collect();
   return x
}
