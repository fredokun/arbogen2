
trait TreeBuilder<T> {
    begin_node : (id : &str) -> T;
    end_node : (node : &mut T, children : Vec<T>) -> ();
    leaf : (id : &str) -> T;
}

enum ToDoElem<T> {
    TodoRule(name : &str),
    TodoNode(node : T)
}

fn gentrees<T>(rng : &mut RandGen, spec : &Spec, rname : &str, builder : TreeBuilder<T>) -> Vec<T> {
    
}

