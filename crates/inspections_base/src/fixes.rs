

enum SemanticChangeStatus {
    Changes,
    NotChanges,
    AlmostNeverChanges,
    Unknown
}

enum BinaryCompatibilityStatus {
    Breaks,
    NotBreaks,
    Unknown
}

enum IntentionAim {
    SettingsChange,
    SolveProblem,

}

trait Intention {
    fn apply() {

    }
}

trait IntentionFamily {

}