/// Lifetime for single pass of inspection
pub struct InspectionEngine<'a> {
//    fn doPass() {
//        EngineState::
//    }
}

impl InspectionEngine {
    // TODO syntax node
    fn run() {

    }
}

struct EngineState<'a> {
    funcs: Vec<Vec<&'a SingleNodeVisitor>>
}

impl<'a> EngineState<'a> {
    pub fn new(funcs: Vec<Vec<&'a SingleNodeVisitor>>) -> Self {
        EngineState { funcs }
    }
}


struct Reporter {

}

struct Context<'a, C> {
    visitor_context: &'a C,

}


/// trait supposed to be called on node of given type (or supertype)
trait SingleNodeVisitor {
    // TODO node
    fn call(&mut self);
}

// TODO C must also have inspection settings
// TODO specify restrictions for C
struct ContextBasedNodeVisitor<'a, C> {
    ctx_ref: &'a C,
    function: fn(&C, Reporter), // TODO add node to function parameter
}

impl SingleNodeVisitor for ContextBasedNodeVisitor<C> {
    fn call(&mut self) {
        self.function.call(self.ctx_ref)
    }
}
