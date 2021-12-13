struct Abiria<'a> {
    name: &'a str,
    bio: &'a str,
    skills: Vec<&'a str>,
    view_count: u64,
}

impl std::fmt::Display for Abiria<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            formatter,
            r##"NAME: [ {name} ]
~ {bio} ~
==================

my skills:
{skills}

 (total {vc} views)"##,
            name = self.name,
            bio = self.bio,
            skills = self
                .skills
                .iter()
                .map(|sk| format!("    - {}", sk))
                .collect::<Vec<String>>()
                .join(",\n"),
            vc = self.view_count
        )
    }
}
fn main() {
    let abiria = Abiria {
        name: "Abiria.dev",
        bio: "js backend developer",
        skills: vec![
            "JavaScript",
            "Rust",
            "Svelte",
            "Typescript",
            "Node.js",
            "Liunx",
        ],
        view_count: 1232_u64,
    };

    println!("{}", abiria);
}
