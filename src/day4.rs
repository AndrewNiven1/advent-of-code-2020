pub fn day4(input_lines: &[String]) -> (u64, u64) {

    let fields: &[String] = &[String::from("byr"),
                            String::from("iyr"),
                            String::from("eyr"),
                            String::from("hgt"),
                            String::from("hcl"),
                            String::from("ecl"),
                            String::from("pid"),
                            String::from("cid"),];

    let mut passport = build_passport();
    let mut passing_passports = 0;

    for index in 0..input_lines.len() {
        
        let line = &input_lines[index];

        for field in fields {
            if line.contains(field) {
                let split1: Vec<&str> = line.split(field).collect();
                let split2: Vec<&str> = split1[1].split(':').collect();
                let split3: Vec<&str> = split2[1].split(' ').collect();
                let update = split3[0];
                // println!("{:?}",split1);
                // println!("{:?}",split2);
                // println!("{:?}",split3);
                // println!("{}",update);
                passport.obligatory_field_count = passport.obligatory_field_count + 1;
                passport = passport.update_field(field,update.to_string())
            }
            
        }

        if line == "" || index == input_lines.len()-1 {
            println!("{}",passport.byr);
            println!("{}",passport.iyr);
            println!("{}",passport.eyr);
            println!("{}",passport.hgt);
            println!("{}",passport.hcl);
            println!("{}",passport.ecl);
            println!("{}",passport.pid);
            println!("{}",passport.cid);
            &passport.check_passport();
            passport = passport.reset_values();
        }
        
    }



    (passing_passports,0)

}

struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
    obligatory_field_count: usize,
}

impl Passport {

    fn reset_values(mut self) -> Passport {
        self.byr =  String::from("n.a.");
        self.iyr =  String::from("n.a.");
        self.eyr =  String::from("n.a.");
        self.hgt =  String::from("n.a.");
        self.hcl =  String::from("n.a.");
        self.ecl =  String::from("n.a.");
        self.pid =  String::from("n.a.");
        self.cid =  String::from("n.a.");
        self.obligatory_field_count = 0;

        self
    }

    fn update_field(mut self, field: &str, update: String) -> Passport {
        match field {
            "byr" => self.byr = update,
            "iyr" => self.iyr = update,
            "eyr" => self.eyr = update,
            "hgt" => self.hgt = update,
            "hcl" => self.hcl = update,
            "ecl" => self.ecl = update,
            "pid" => self.pid = update,
            "cid" => self.cid = update,
            _     => println!{"{}","Error"}
        }

        self
    }

    fn check_passport(&self) -> bool {
        let mut byr_valid = false;
        let mut iyr_valid = false;
        let mut eyr_valid = false;
        let mut hgt_valid = false;
        let mut hcl_valid = false;
        let mut ecl_valid = false;
        let mut pid_valid = false;
        let mut cid_valid = true;

        let byr_int = match self.byr.parse::<i32>() {
            Ok(byr_int) => byr_int,
            Err(e) => 0,
        };
        if byr_int >= 1920 && byr_int <= 2002 {
            byr_valid = true;
        }

        let iyr_int = match self.iyr.parse::<i32>() {
            Ok(iyr_int) => iyr_int,
            Err(e) => 0,
        };
        if iyr_int >= 2010 && iyr_int <= 2020 {
            iyr_valid = true;
        }

        let eyr_int = match self.eyr.parse::<i32>() {
            Ok(eyr_int) => eyr_int,
            Err(e) => 0,
        };
        if eyr_int >= 2010 && eyr_int <= 2020 {
            eyr_valid = true;
        }

        false
    }

}

fn build_passport() -> Passport {
    Passport {
        byr: String::from("n.a."),
        iyr: String::from("n.a."),
        eyr: String::from("n.a."),
        hgt: String::from("n.a."),
        hcl: String::from("n.a."),
        ecl: String::from("n.a."),
        pid: String::from("n.a."),
        cid: String::from("n.a."),
        obligatory_field_count: 0,
    }
}
