use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

#[derive(Debug)]
pub struct WorkflowStep<'a> {
    pub part: usize,
    pub ordering: Ordering,
    pub value: u64,
    pub destination: &'a str
}

impl<'a> From<&'a str> for WorkflowStep<'a> {
    fn from(value: &'a str) -> Self {
        let (value, destination) = value.rsplit_once(":").unwrap();
        let mut chars = value.chars();
        let part = match chars.nth(0).unwrap() {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => panic!("wtf")
        };
        let ordering = match chars.nth(0).unwrap() {
            '>' => Ordering::Greater,
            '<' => Ordering::Less,
            _ => panic!("wtf")
        };
        Self {
            part: part,
            ordering: ordering,
            value: chars.collect_vec().into_iter().collect::<String>().parse().unwrap(),
            destination: destination
        }
    }
}


#[derive(Debug)]
pub struct WorkflowSteps<'a> {
    pub steps: Vec<WorkflowStep<'a>>,
    pub destination: &'a str
}

impl<'a> From<&'a str> for WorkflowSteps<'a> {
    fn from(value: &'a str) -> Self {
        let (steps, desintation) = value.rsplit_once(",").unwrap();
        let workflow_steps = steps.split(",").map(|s| WorkflowStep::from(s)).collect_vec();
        Self {
            steps: workflow_steps,
            destination: desintation
        }
    }
}


pub struct Workflows<'a>(pub HashMap<&'a str, WorkflowSteps<'a>>);
impl<'a> From<&'a str> for Workflows<'a> {
    fn from(value: &'a str) -> Self {
        let mut workflows: HashMap<&str, WorkflowSteps> = HashMap::new();

        for workflow in value.lines() {
            let mut workflow_split = workflow.split("{");
            let key = workflow_split.nth(0).unwrap();
            let mut step_chars = workflow_split.nth(0).unwrap().chars();
            step_chars.next_back();
            let steps = step_chars.as_str();
            workflows.insert(key, WorkflowSteps::from(steps));
        }

        Self(workflows)
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Range {
    pub min: u64,
    pub max: u64
}


pub fn get_next_destination<'a>(part: Vec<u32>, workflow: &'a WorkflowSteps) -> &'a str {
    for step in workflow.steps.iter() {
        if (part[step.part] as u64).cmp(&step.value) == step.ordering {
            return step.destination
        }
    }
    return workflow.destination
}
