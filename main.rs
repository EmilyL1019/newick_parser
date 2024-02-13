use newick::parser::create_final_tree;
use newick::parser::Tree;
use newick::parser::print_tree;

fn main() {
    //let tree = newick::parser::create_tree("(X06485:0.05467,X53853:0.06192)1.000:0.16927".t{o_string());
    let mut newick: String = "(((((1544310:0.00081,49435:0.0033):0.00014,347846:0.00081):0.00014,562859:0.00079):0.00078,109254:0.00161):0.0056,140691:0.00399)".to_string();
    let tree = create_final_tree(newick);
    println!("Tree 1");
    print_tree::<String>(tree);

    /*let tree1 = newick::parser::get_small_trees("(((((1544310:0.00081,49435:0.0033):0.00014,347846:0.00081):0.00014,562859:0.00079):0.00078,109254:0.00161):0.0056,140691:0.00399)".to_string());
    //Should be (1544310:0.00081,49435:0.0033):0.00014
    let d_left_string: String = match tree1.Left.Distance {
        Some(x) => x.to_string(),
        None => "No distance".to_string(), 
    }
    let d_right_string: String = match tree1.Right.Distance {
        Some(x) => x.to_string(),
        None => "No distance".to_string(), 
    }
    println!("{} (} {}", tree1.Left.Value, d_left_string, tree1.Left.Index)
    println!("{} (} {}", tree1.Right.Value, d_right_string, tree1.Right.Index)
    let dstring: String = match tree1.Distance {
        Some(x) => x.to_string(),
        None => "No distance".to_string(), 
    }
    let pstring: String = match tree1.Probability {
        Some(x) => x.to_string(),
        None => "No probability".to_string(), 
    }
    println!("{} {} {} {}", dstring, pstring, tree1.Index, tree1.Id)*/
}