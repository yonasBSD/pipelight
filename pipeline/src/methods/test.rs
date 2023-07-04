#[cfg(test)]
mod trigger_match {
    use crate::{Trigger, TriggerBranch, TriggerTag};
    use utils::git::{Flag, Hook};
    // Error Handling
    use miette::{IntoDiagnostic, Result};

    #[test]
    /// match trigger with branch without action
    fn try_match_branch() {
        let env = Trigger::TriggerBranch(TriggerBranch {
            branch: Some("master".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
        });
        let triggers = vec![Trigger::TriggerBranch(TriggerBranch {
            branch: Some("master".to_owned()),
            action: None,
        })];
        assert_eq!(
            env.is_branch_match(triggers.first().unwrap().to_owned())
                .is_ok(),
            true
        );
        assert_eq!(
            env.is_action_match(triggers.first().unwrap().to_owned())
                .is_ok(),
            true
        );
        assert_eq!(env.is_match(triggers).is_ok(), true);
    }
    #[test]
    /// unmatch trigger with branch without action
    fn try_unmatch_branch() {
        let env = Trigger::TriggerBranch(TriggerBranch {
            branch: Some("master".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
        });
        let triggers = vec![Trigger::TriggerBranch(TriggerBranch {
            branch: Some("dev".to_owned()),
            action: None,
        })];
        assert_eq!(env.is_match(triggers).is_ok(), false);
    }
    #[test]
    /// match trigger with action without branch
    fn try_match_action() {
        let env = Trigger::TriggerBranch(TriggerBranch {
            branch: Some("master".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
        });
        let triggers = vec![Trigger::TriggerBranch(TriggerBranch {
            branch: None,
            action: Some(Flag::Hook(Hook::PrePush)),
        })];
        assert_eq!(env.is_match(triggers).is_ok(), true);
    }
    #[test]
    /// unmatch trigger with action without branch
    fn try_unmatch_action() {
        let env = Trigger::TriggerBranch(TriggerBranch {
            branch: Some("master".to_owned()),
            action: Some(Flag::Watch),
        });
        let triggers = vec![Trigger::TriggerBranch(TriggerBranch {
            branch: None,
            action: Some(Flag::Hook(Hook::PrePush)),
        })];
        assert_eq!(env.is_match(triggers).is_ok(), false);
    }
    #[test]
    /// match trigger with tag without action
    fn try_match_tag() {
        let env = Trigger::TriggerTag(TriggerTag {
            tag: Some("v0.4".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
        });
        let triggers = vec![Trigger::TriggerTag(TriggerTag {
            tag: Some("v0.4".to_owned()),
            action: None,
        })];
        assert_eq!(env.is_match(triggers).is_ok(), true);
    }
    #[test]
    /// unmatch trigger with tag without action
    fn try_unmatch_tag() {
        let env = Trigger::TriggerTag(TriggerTag {
            tag: Some("v0.5".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
        });
        let triggers = vec![Trigger::TriggerTag(TriggerTag {
            tag: Some("v0.4".to_owned()),
            action: None,
        })];
        assert_eq!(env.is_match(triggers).is_ok(), false);
    }
    #[test]
    /// match trigger enums
    fn try_match_enum() {
        let env = Trigger::TriggerTag(TriggerTag {
            tag: Some("v0.5".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
        });
        let triggers = vec![Trigger::TriggerBranch(TriggerBranch {
            branch: None,
            action: None,
        })];
        assert_eq!(env.is_match(triggers).is_ok(), true);
    }
    #[test]
    fn try_match_action_no_tag() {
        let env = Trigger::TriggerTag(TriggerTag {
            tag: Some("v0.5".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
        });
        let triggers = vec![Trigger::TriggerTag(TriggerTag {
            tag: None,
            action: Some(Flag::Hook(Hook::PrePush)),
        })];
        assert_eq!(env.is_match(triggers).is_ok(), true);
    }
    #[test]
    fn try_unmatch_action_no_tag() {
        let env = Trigger::TriggerTag(TriggerTag {
            tag: Some("v0.5".to_owned()),
            action: Some(Flag::Hook(Hook::PrePush)),
        });
        let triggers = vec![Trigger::TriggerTag(TriggerTag {
            tag: None,
            action: Some(Flag::Hook(Hook::PreCommit)),
        })];
        assert_eq!(env.is_match(triggers).is_ok(), false);
    }
}
