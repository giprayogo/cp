use std::collections::{BTreeSet, HashMap};

struct Video {
    pub video: Vec<char>,
    pub like: i32,
    pub dislike: i32,
    pub views: i32,
}

impl Video {
    fn new(video: String) -> Self {
        let video = video.chars().collect();
        Self {
            video,
            like: 0,
            dislike: 0,
            views: 0,
        }
    }
}

struct VideoSharingPlatform {
    videos: HashMap<i32, Video>,
    deleted: BTreeSet<i32>,
    next_id: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl VideoSharingPlatform {
    fn new() -> Self {
        Self {
            videos: HashMap::new(),
            deleted: BTreeSet::new(),
            next_id: 0,
        }
    }

    fn upload(&mut self, video: String) -> i32 {
        let id = match self.deleted.pop_first() {
            Some(v) => v,
            None => {
                let v = self.next_id;
                self.next_id += 1;
                v
            }
        };
        self.videos.insert(id, Video::new(video));
        id
    }

    fn remove(&mut self, video_id: i32) {
        match self.videos.remove(&video_id) {
            Some(_) => self.deleted.insert(video_id),
            None => false,
        };
    }

    fn watch(&mut self, video_id: i32, start_minute: i32, end_minute: i32) -> String {
        if let Some(x) = self.videos.get_mut(&video_id) {
            let mut start_minute = start_minute as usize;
            let mut end_minute = end_minute as usize;
            start_minute = start_minute.min(x.video.len() - 1);
            end_minute = end_minute.min(x.video.len() - 1);
            x.views += 1;
            x.video[start_minute..=end_minute].iter().collect()
        } else {
            "-1".to_string()
        }
    }

    fn like(&mut self, video_id: i32) {
        if let Some(x) = self.videos.get_mut(&video_id) {
            x.like += 1;
        }
    }

    fn dislike(&mut self, video_id: i32) {
        if let Some(x) = self.videos.get_mut(&video_id) {
            x.dislike += 1;
        }
    }

    fn get_likes_and_dislikes(&self, video_id: i32) -> Vec<i32> {
        if let Some(x) = self.videos.get(&video_id) {
            vec![x.like, x.dislike]
        } else {
            vec![-1]
        }
    }

    fn get_views(&self, video_id: i32) -> i32 {
        if let Some(x) = self.videos.get(&video_id) {
            x.views
        } else {
            -1
        }
    }
}

/**
 * Your VideoSharingPlatform object will be instantiated and called as such:
 * let obj = VideoSharingPlatform::new();
 * let ret_1: i32 = obj.upload(video);
 * obj.remove(videoId);
 * let ret_3: String = obj.watch(videoId, startMinute, endMinute);
 * obj.like(videoId);
 * obj.dislike(videoId);
 * let ret_6: Vec<i32> = obj.get_likes_and_dislikes(videoId);
 * let ret_7: i32 = obj.get_views(videoId);
 */
fn main() {
    let mut obj = VideoSharingPlatform::new();
    let id_0 = obj.upload("ABCDEFG".to_string());
    obj.upload("XYZ".to_string());
    assert_eq!(id_0, 0);

    obj.remove(id_0);
    obj.remove(2);
    let mut vid = obj.watch(0, 0, 1);
    assert_eq!(vid, "-1");

    let id_1 = obj.upload("ABCDEFG".to_string());
    assert_eq!(id_1, 0);
    let id_2 = obj.upload("HIJK".to_string());
    assert_eq!(id_2, 2);

    vid = obj.watch(0, 0, 1);
    assert_eq!(vid, "AB");
    vid = obj.watch(1, 0, 5);
    assert_eq!(vid, "XYZ");
    vid = obj.watch(2, 0, 3);
    assert_eq!(vid, "HIJK");

    obj.like(0);
    obj.dislike(1);
    assert_eq!(obj.get_likes_and_dislikes(0), [1, 0]);
    assert_eq!(obj.get_likes_and_dislikes(1), [0, 1]);
    assert_eq!(obj.get_views(0), 1);
    assert_eq!(obj.get_views(1), 1);
    assert_eq!(obj.get_views(2), 1);
    // let ret_3 = obj.watch(0, 0, 1);
    // the complex thing is that the index can be reusd: how to implement this efficiently?
    // should I have a (sorted) list of deleted and added index? in that case then
    // Pair of btreeset?
    // log(n) check
    // log(n) insert del everyting
    // no -> for main video I should use hashmap
    // for deleted -then- I would use the btreeset or min heap
    // btreemap with value of {video: Vec<char>, like: i32, dislike: i32}
    // because this is rust and I would like to slice the video at some point, I should save the video internally as Vec<char>
}
