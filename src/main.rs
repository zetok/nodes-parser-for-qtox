/*
    Copyright © 2015 Zetok Zalbavar <zetok@openmailbox.org>

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/


use std::io::Read;
use std::fs::File;


/*
    Function to read file and return vector of strings, each of them
    corresponding to a line from a file.

    In a case where there is no file, return early.
*/
fn vec_strings(file: &str) -> Result<Vec<String>, ()> {
    let mut file = match File::open(file) {
        Ok(f) => f,
        Err(e) => {
            println!("Error opening {}: {}", file, e);
            return Err(())
        },
    };

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    Ok(content.lines().map(|l| l.to_string()).collect())
}


fn bootstrap(string: &String, num: usize) -> Option<String> {
    let split: Vec<&str> = string.split_whitespace().collect();
    if split.len() >= 4 {
        let mut name = String::new();
        for n in &split[3..] {
            name.push_str(n);
            name.push_str(" ");
        }
        // 0 - number
        // 1 - IP
        // 2 - port
        // 3 - pubkey
        // 4 - name
        let node = format!(
"dhtServerList\\{0}\\name={4}
dhtServerList\\{0}\\userId={3}
dhtServerList\\{0}\\address={1}
dhtServerList\\{0}\\port={2}",
            num,      // 0
            split[0], // 1
            split[1], // 2
            split[2], // 3
            name      // 4
        );
        Some(node)
    } else { None }
}


fn main() {
    let file = "nodes_working";
    let nodes = match vec_strings(file) {
        Ok(v) => v,
        Err(_) => return,
    };

    let mut to_return: Vec<String> = vec![];

    to_return.push(format!("dhtServerList\\size={}", nodes.len()));

    for n in 0..nodes.len() {
        if let Some(node) = bootstrap(&nodes[n], n + 1) {
            to_return.push(node);
        }
    }

    for node in to_return {
        println!("{}", node);
    }
}
