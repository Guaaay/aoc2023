
use std::{collections::HashMap as Map};

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<&'a str>,
}

#[derive(Debug)]
struct Part{
    map: Map<char, i32>,
}

fn parse_workflows(lines: &str) -> Map<String, Workflow> {
    let mut workflows = Map::new();
    let mut workflow_lines: Vec<&str> = lines.split("\n\n").collect::<Vec<&str>>()[0].split("\n").collect();
    //println!("{:?}", workflow_lines);
    for workflow_line in workflow_lines {
        let parsed = workflow_line.split("{").collect::<Vec<&str>>();
        let name = parsed[0].trim();
        let mut rules_str = parsed[1].split(",").collect::<Vec<&str>>();
        workflows.insert(
            name.to_string(),
            Workflow {
                name: name,
                rules: rules_str,
            },
        );
    
    }
    workflows
}

fn parse_parts(lines: &str)-> Vec<Part>{
    let mut parts: Vec<Part> = Vec::new();
    let part_lines: Vec<&str> = lines.split("\n\n").collect::<Vec<&str>>()[1].split("\n").collect();
    for part_line in part_lines {
        let parsed = part_line.split(",").collect::<Vec<&str>>();
        let mut part = Part{
            map: Map::new(),
        };
        println!("{:?}", parsed);
        part.map.insert('x',parsed[0].split("=").collect::<Vec<&str>>()[1].trim().parse::<i32>().unwrap()); 
        part.map.insert('m',parsed[1].split("=").collect::<Vec<&str>>()[1].trim().parse::<i32>().unwrap());
        part.map.insert('a',parsed[2].split("=").collect::<Vec<&str>>()[1].trim().parse::<i32>().unwrap());
        part.map.insert('s',parsed[3].split("=").collect::<Vec<&str>>()[1].split("}").collect::<Vec<&str>>()[0].trim().parse::<i32>().unwrap());
        
        parts.push(part);
    }
    //&&println!("{:?}", parts);
    parts
}

fn evaluate_rule(part: &Part, rule: &str) -> i16{
    //println!("rule: {:?} ", rule);

    //We parse which attribute we are evaluating
    let attribute = rule.chars().nth(0).unwrap();
    //println!("Att: {:?}", attribute);
    if attribute != 'x' && attribute != 'm' && attribute != 'a' && attribute != 's'{
        return 2;
    }
    let val = part.map.get(&attribute).unwrap();
    //We parse the operator
    let operator = rule.chars().nth(1).unwrap();
    //println!("Op: {:?}", operator);
    //println!("Val: {:?}", val);
    //println!("comp {:?}",  rule.split(">").collect::<Vec<&str>>()[1].split(":").collect::<Vec<&str>>()[0].trim().parse::<i32>().unwrap());
    if operator == '>' 
    {
        //println!("comp {:?}",  rule.split(">").collect::<Vec<&str>>()[1].split(":").collect::<Vec<&str>>()[0].trim().parse::<i32>().unwrap());
        return (*val > rule.split(">").collect::<Vec<&str>>()[1].split(":").collect::<Vec<&str>>()[0].trim().parse::<i32>().unwrap()) as i16;
    } 
    else if operator == '<' 
    {
        //println!("comp {:?}",  rule.split("<").collect::<Vec<&str>>()[1].split(":").collect::<Vec<&str>>()[0].trim().parse::<i32>().unwrap());
        return (*val < rule.split("<").collect::<Vec<&str>>()[1].split(":").collect::<Vec<&str>>()[0].trim().parse::<i32>().unwrap()) as i16;
    }
    return 2;
}

fn add_ratings(part: &Part) -> i32 {
    let mut sum = 0;
    for (_, val) in part.map.clone() {
        sum += val;
    }
    sum
}

fn part1(lines: &str) {
    let workflows = parse_workflows(&lines);
    let parts = parse_parts(&lines);
    let mut sum = 0;
    for part in &parts {
        println!("{:?}", part);
        let mut workflow = workflows.get("in").unwrap();
        let done = false;
        //println!("{:?}", part);
        while !done {
            println!("{:?}", workflow);
            let mut state = 0;
            for rule in &workflow.rules {
                
                let rule_new = rule.split('}').collect::<Vec<&str>>()[0].trim();
                //println!("rule: {:?}", rule_new);
                if rule_new.chars().nth(0).unwrap() == 'A'{
                    state = 1;
                    break;
                }
                if rule_new.chars().nth(0).unwrap() == 'R'{
                    state = 2;
                    break;
                }
                let res = evaluate_rule(part, rule_new);
                println!("res: {:?}", res);
                if res == 1 {
                    let target = rule_new.split(":").collect::<Vec<&str>>()[1].trim();
                    if target == "A" {
                        state = 1;
                        break;
                    }
                    if target == "R" {
                        state = 2;
                        break;
                    }
                    //println!("xd: {:?}", target);
                    workflow = workflows.get(target.trim()).unwrap();
                    break;
                }
                else if res == 2 {
                    //println!("Error: {:?}", rule_new);
                    workflow = workflows.get(rule_new.trim()).unwrap();
                }
            }
            if state == 1 {
                sum += add_ratings(&part);
                break;
            }
            else if state == 2 {
                break;
            }
            
        }
        
    }
    println!("{:?}", sum);
}

fn main() {
    let lines = include_str!("../example.txt");
    part1(&lines);
}
