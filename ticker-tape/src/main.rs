#![allow(warnings)]
use std::io::Cursor;

const COLS: usize = 64usize;
const SEP: usize = 3usize;

fn main() {


let head =    r#"

<html>
<head>

<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Aldrich&family=Jura:wght@300..700&family=Nabla:EDPT,EHLT@0,0&family=Space+Grotesk:wght@300..700&family=Space+Mono:ital,wght@0,400;0,700;1,400;1,700&display=swap" rel="stylesheet">


<style>


#skills {
    font-family: "Space Mono", monospace;
    font-weight: 400; font-style: normal;
}

.title {
    font-family: "Space Mono", monospace;
    font-style: bold;
}

</style>

<body>
<section id="skills">
    "#;

    println!("{}",head);



    let mut curse = Curse::default();

    let languages = r#"Rust,Java,Dart,HTML5,TypeScript,JavaScript,Golang,Python"#;
    let frameworks = r#"Spring Boot,Flutter,Angular,Hugo,RedHat Operator SDK"#;

    let os = r#"
Mac,Windows,Linux,Ubuntu,RedHat Enterprise,Alpine,iOS,Android"#;



    let databases = r#"
PostgreSQL,Neo4j,MongoDB"#;




    let ops = r#"Docker,Kubernetes,Helm,Skaffold,Terraform,Kibana,Beats,Logstash,Prometheus,Zipkin,Grafana,Jaeger"#;
    let cicd = r#"
Jenkins,Concourse CI,GitHub Actions,Blu-Ray Disc development (BD-J)"#;

    let gcp = r#"Compute Engine,GKE,Buckets,Pub/Sub,Image Registry,IAM,GKE Ingress with managed certs"#;

    let aws = r#"EC2 S3, lambda, route 53, SQS, SNS Beanstalk IAM"#;


    let misc = "Redis,Memcached,RabbitMQ,Kafka,Git,GitHub Actions,ElasticSearch,Solr";

    curse.named_section("GCP", gcp);
    curse.named_section("AWS", aws);
    curse.named_section("OPS", ops);
    curse.named_section("OS", os);
    curse.named_section("CI/CD", cicd);
    curse.named_section("LANG", languages);
    curse.named_section("MISC", misc);
    curse.fill(100);


    let index = 0;

    let cursor = Curse::default();

    println!("\n</section>");
    println!("\n\n\n");
    println!("</body></html>");
}


pub struct Curse {
    pub line: usize,
    pub index: usize,
}

impl Curse {
    pub fn named_section(&mut self, name: &str, content: &str ) {

        println!("<span class=\"cat {name}\">");

        self.push_title(name.trim());
        for item in content.clone().trim().split(",") {
            self.push(item.trim());
        }
        println!("</span>");
        self.newline()
    }


    pub fn section(&mut self, name: &str, content: &str ) {

        print!("<span class=\"cat\">");

        for item in content.clone().trim().split(",") {
            self.push(item.trim());
        }
        print!("</span>");
    }

    pub(self) fn push_title( &mut self, name: &str ) {
        if self.index + name.len() > COLS {
            self.newline();
        }
        print!("<span class=\"title\">{name}</span>");
        self.index += name.len();
        self.fill(3);
    }

    pub(self) fn push( &mut self, name: &str ) {
        if self.index + name.len() > COLS {
            self.newline();
            self.fill(3);
        }
        print!("<span>{name}</span>");
        self.index += name.len();
        self.fill(3);
    }

    fn newline(&mut self) {
        if COLS > self.index {
            self.fill( COLS-self.index );
        }
        print!("<br>");
        self.line += 1;
        self.index = 0;
    }

    fn fill( & mut self, mut len: usize ) {
        if self.index + len > COLS {
            self.newline();
            return;
        }
        print!("<span class=\"dots\">{}</span>", ".".repeat(len));
        self.index += len;
    }


}

impl Default for Curse {
    fn default() -> Self {
       Self {
           line: 0,
           index: 0,
       }
    }
}

