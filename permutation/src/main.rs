
struct Permutation{
    original:Vec<i32>,
    per:Vec<i32>,
    chosen:Vec<bool>,
    result:Vec<Vec<i32>>
}
impl Permutation{
    fn new(original:&Vec<i32>) -> Permutation{
        let mut result: Permutation = Permutation { 
            original: original.clone(), 
            per: Vec::with_capacity(original.len()), 
            chosen: Vec::with_capacity(original.len()), 
            result: Vec::with_capacity(original.len()*2) };
        result.chosen.resize(original.len(), false);

        return result;
    }

    fn search(&mut self){
        if self.per.len() == self.original.len() {
            self.result.push(self.per.clone());
            //self.per.clear();
        }
        else{
            for i in 0..self.original.len(){
                if self.chosen[i] { continue; }
                self.chosen[i] = true;
                self.per.push(self.original[i]);
                self.search();
                self.chosen[i] = false;
                self.per.remove(self.per.len()-1);
            }
        }
    }
}

fn main() {
    let v: Vec<i32> = vec![0,1,2];
    let mut per: Permutation = Permutation::new(&v);
    per.search();
    println!("original Vector: {:?}",v);
    println!("permutation is: {:?}", per.result);
}
