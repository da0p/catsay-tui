use cursive::traits::Nameable;
use cursive::views::{Checkbox, Dialog, EditView, ListView};
use cursive::Cursive;

// Wrap all form fields value in one struct
// so we can pass them around easily
struct Options<'a> {
    message: &'a str,
    dead: bool,
}

fn main() {
    let mut siv = cursive::default();

    input_step(&mut siv);

    siv.run();
}

fn input_step(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Please fill out the form for the cat")
            .content(
                ListView::new()
                    .child("Message:", EditView::new().with_name("message"))
                    .child("Dead?", Checkbox::new().with_name("dead")),
            )
            .button("OK", |s| {
                let message = s
                    .call_on_name("message", |s: &mut EditView| s.get_content())
                    .unwrap();
                let is_dead = s
                    .call_on_name("dead", |t: &mut Checkbox| t.is_checked())
                    .unwrap();
                let options = Options {
                    message: &message,
                    dead: is_dead,
                };
                result_step(s, &options)
            }),
    );
}

fn result_step(siv: &mut Cursive, options: &Options) {
    let eye = if options.dead { "x" } else { "o" };

    let cat_text = format!(
        "{msg}
 \\
  \\
    /\\_/\\
   ( {eye} {eye} )
   =( I )=",
        msg = options.message,
        eye = eye
    );

    siv.pop_layer();
    siv.add_layer(
        Dialog::text(cat_text)
            .title("The cat says...")
            .button("OK", |s| s.quit()),
    );
}
