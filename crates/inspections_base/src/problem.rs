use text_unit::TextRange;

pub enum ProblemType {
    CodeStyle,
    Performance,
    ProbableBugs,
    BadPractice,
}

pub enum Confidence {
    AlwaysSound,
    AlwaysUnsound,
    DependsOnCase
}

// TODO problem number?
struct ProblemDescription {
    problem_type: ProblemType,
    confidence: Confidence,
    special_marks: Vec<String>,
    should_be_available_after_apply: bool,
    applicable_for_batch: bool,
}

trait Problem {
    fn get_problem_description() -> ProblemDescription;

    fn suggest_fixes() -> Vec<IntentionFamily>;

    fn ranges() -> Vec<TextRange>;
}