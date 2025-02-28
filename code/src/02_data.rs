pub struct DistillationColumn {
    trays: i32,
    feed_place: i32,
    reflux_ratio: f32,
    distiliate_to_feed_ratio: f32,
}

fn main() {
    let stages = 20;
    let feed_place = 10;
    let reflux_ratio = 1.5;
    let d2f = 0.9;
    let ds = DistillationColumn {
        trays: stages,                 // different names
        feed_place,                    // var name as field name
        reflux_ratio,                  // var name as field name
        distiliate_to_feed_ratio: d2f, // different names
    };

    ref_ds(&ds);
    move_ds(ds);
}

fn move_ds(ds: DistillationColumn) {
    println!(
        "MOVE: Distillation Column has {} trays and feeds at {} and operaters with d2f={}, and rr={}",
        ds.trays, ds.feed_place, ds.distiliate_to_feed_ratio, ds.reflux_ratio
    );
}

fn ref_ds(ds: &DistillationColumn) {
    println!(
        "REF:  Distillation Column has {} trays and feeds at {} and operaters with d2f={}, and rr={}",
        ds.trays, ds.feed_place, ds.distiliate_to_feed_ratio, ds.reflux_ratio
    );
}
