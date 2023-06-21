//1 
fn main() {
    println!("The Entrepreneurial Experience!");
}

//2
fn setup() {
    let current_goal = String::from("Turning an idea into a business");
    println!("Current Goal: {}", current_goal);
}

//3
fn plans() {
    let plan_steps = vec![
        "Researching the market", 
        "Developing a plan", 
        "Creating a budget",
        "Finding investors", 
        "Building a team"
    ];

    for step in plan_steps {
        println!("{}", step);
    }
}

//4
fn resources() {
    let resources = [
        "Mentorship programs",
        "Startup incubators",
        "Crowdfunding platforms",
        "Business plan templates"
    ];

    println!("Resources one might use:");
    for resource in resources.iter() {
        println!("  - {}", resource);
    }
}

//5
fn challenges(challenges: Vec<&str>) {
    println!("Common challenges:");
    for challenge in challenges {
        println!("  - {}", challenge);
    }
}

//6
fn success_stories() {
    let entrepreneurs = [
        ("Steve Jobs", "Apple"),
        ("Evan Spiegel", "Snapchat"),
        ("Mark Zuckerberg", "Facebook")
    ];

    println!("Inspirational entrepreneurs:");
    for entrepreneur in entrepreneurs.iter() {
        println!("  - {}: {}", entrepreneur.0, entrepreneur.1);
    }
}

//7
fn advice() {
    let tips = vec![
        "Believe in your idea",
        "Stay organized",
        "Stay patient",
        "Stay focused"
    ];

    println!("Tips to keep in mind:");
    for tip in tips {
        println!("  - {}", tip);
    }

}

//8
fn growth() {
    let milestones = [
        ("First Sale", 1000),
        ("5,000 Sales", 50000),
        ("20,000 Sales", 200000)
    ];
    
    println!("Milestones to reach:");
    for milestone in milestones.iter() {
        println!("  - {}: {}", milestone.0, milestone.1);
    }
}

//9
fn mistakes() {
    let mistakes = vec![
        "Giving up too soon",
        "Not delegating tasks",
        "Failing to network"
    ];

    println!("Common mistakes to avoid:");
    for mistake in mistakes {
        println!("  - {}", mistake);
    }
}

//10
fn conclusion() {
    println!("The entrepreneurial experience is never easy, but it can be rewarding if done right. Take the knowledge learned in this lesson and go make a great company!");
}

//11
fn main() {
    println!("The Entrepreneurial Experience!");
    setup();
    plans();
    resources();
    challenges(vec!["Lack of funding", "Lack of resources", "Lack of support"]);
    success_stories();
    advice();
    growth();
    mistakes();
    conclusion();
}