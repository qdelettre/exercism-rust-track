use fluent::{concurrent::FluentBundle, FluentArgs, FluentResource};
use std::{
    env,
    fmt::{Display, Formatter, Result},
    fs,
};

use unic_langid::{langid, LanguageIdentifier};

static L10N_RESOURCE: &str = "beer-song.ftl";
static LOCALE: LanguageIdentifier = langid!("en-US");

struct Verse {
    max: u32,
    n: u32,
    bundle: FluentBundle<FluentResource>,
}

impl Display for Verse {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut args = FluentArgs::new();
        args.add("max", self.max.into());
        args.add("count", self.n.into());

        match self.n.checked_sub(1) {
            Some(next_count) => args.add("nextCount", next_count.into()),
            None => {}
        }

        match &self.n {
            0 => {
                let no_more_bottles_msg = self
                    .bundle
                    .get_message("no-more-bottles")
                    .expect("Message doesn't exist.");
                let mut errors = vec![];
                let pattern = no_more_bottles_msg.value.expect("Message has no value.");
                let no_more_bottles_value =
                    self.bundle
                        .format_pattern(&pattern, Some(&args), &mut errors);

                let go_to_store_msg = self
                    .bundle
                    .get_message("go-to-store")
                    .expect("Message doesn't exist.");
                let mut errors = vec![];
                let pattern = go_to_store_msg.value.expect("Message has no value.");
                let go_to_store_value =
                    self.bundle
                        .format_pattern(&pattern, Some(&args), &mut errors);

                write!(f, "{}.\n{}.\n", no_more_bottles_value, go_to_store_value)
            }
            _n => {
                let bottles_on_the_wall_msg = self
                    .bundle
                    .get_message("bottles_on_the_wall")
                    .expect("Message doesn't exist.");
                let mut errors = vec![];
                let pattern = bottles_on_the_wall_msg
                    .value
                    .expect("Message has no value.");
                let bottles_on_the_wall_value =
                    self.bundle
                        .format_pattern(&pattern, Some(&args), &mut errors);

                let take_down_msg = self
                    .bundle
                    .get_message("take_down")
                    .expect("Message doesn't exist.");
                let mut errors = vec![];
                let pattern = take_down_msg.value.expect("Message has no value.");
                let take_down_msg_value =
                    self.bundle
                        .format_pattern(&pattern, Some(&args), &mut errors);

                write!(
                    f,
                    "{}.\n{}.\n",
                    bottles_on_the_wall_value, take_down_msg_value
                )
            }
        }
    }
}

pub fn verse(n: u32) -> String {
    let mut bundle = FluentBundle::new(vec![LOCALE.clone()]);

    let mut full_path = env::current_dir().expect("Failed to retrieve current dir.");
    full_path.push("src/resources");
    full_path.push(LOCALE.to_string());
    full_path.push(L10N_RESOURCE);
    let source = fs::read_to_string(full_path).expect("Failed to read file.");
    let resource = FluentResource::try_new(source).expect("Could not parse an FTL string.");
    bundle
        .add_resource(resource)
        .expect("Failed to add FTL resources to the bundle.");

    bundle.set_use_isolating(false); // Remove FSI/PDF isolation marks

    let verse = Verse { max: 99, n, bundle };
    verse.to_string()
}

pub fn sing(start: u32, end: u32) -> String {
    (end..start + 1)
        .rev()
        .into_iter()
        .fold(String::from(""), |mut i, j| {
            i.push_str(&verse(j));
            if j != end {
                i.push_str("\n");
            }
            i
        })
}
