// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "git")]
struct Git {
    #[command(subcommand)]
    cmd: Subcommands,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    #[command(subcommand)]
    Remote(RemoteCommand),
}

#[derive(Parser, Debug)]
enum RemoteCommand {
    Add(AddCommand),
}

#[derive(Parser, Debug, Default, Clone)]
struct AddCommand {
    name: String,
    url: String,
}

#[test]
fn test_no_parse() {
    let result = Git::try_parse_from(["git", "remote", "add", "origin", "git://lol.git"]);
    assert!(result.is_ok());
    let result = Git::try_parse_from(["git", "remote", "ad", "origin", "git://lol.git"]);
    assert!(result.is_err());
    let error = result.unwrap_err();
    println!("Does this error say that it's on 'remote' or on 'ad'? {error:#?}");
    println!("If that says the invalid command is 'ad', then clap is working properly, and the error is on our side.");
}
