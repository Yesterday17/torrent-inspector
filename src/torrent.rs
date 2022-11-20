use serde::{Deserialize, Serialize};

/// Metainfo files (also known as .torrent files) are bencoded dictionaries with the following keys:
///
/// [https://fbdtemme.github.io/torrenttools/topics/bittorrent-metafile-v1.html]
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Torrent {
    /// The URL of the tracker
    #[serde(default)]
    pub announce: Option<String>,
    /// Info dictionary
    pub info: Info,

    /// Multi-tracker Metadata Extension
    ///
    /// This extensions adds support for multiple trackers. A new key **announce-list** is added.
    ///
    /// The tiers of announces will be processed sequentially; all URLs in each tier must be checked before the client
    /// goes on to the next tier. URLs within each tier will be processed in a randomly chosen order;
    /// in other words, the list will be shuffled when first read, and then parsed in order.
    /// In addition, if a connection with a tracker is successful, it will be moved to the front of the tier.
    ///
    /// The **announce-list** key in JSON for a single tier per tracker:
    /// ```text
    /// "announce-list" : [
    ///     [ "tracker1" ],
    ///     [ "backup1" ],
    ///     [ "backup2" ]
    /// ]
    /// ```
    ///
    /// On each announce, first try tracker1, then if that cannot be reached, try backup1 and backup2 respectively.
    /// On the next announce, repeat in the same order. This is meant for when the trackers are standard and can not share information.
    ///
    /// The **announce-list** key in JSON with multiple trackers in a tier:
    /// ```text
    /// "announce-list": [
    ///     [ "tracker1", "tracker2"],
    ///     [ "backup1" ]
    /// ]
    /// ```
    ///
    /// This form is meant for trackers which can trade peer information and will cause the clients to help balance
    /// the load between the trackers. The first tier, consisting of tracker1 and tracker2, is shuffled.
    /// Both trackers 1 and 2 will be tried on each announce (though perhaps in varying order) before the client tries to reach backup1.
    #[serde(default)]
    #[serde(rename = "announce-list")]
    pub announce_list: Vec<Vec<String>>,

    /// DHT protocol
    ///
    /// DHT is a way to share peers ina swarm without a centralized tracker.
    /// A trackerless torrent dictionary does not have an announce key.
    /// Instead, a trackerless torrent has a “nodes” key. This key should be set to
    /// the K closest nodes in the torrent generating client’s routing table.
    /// Alternatively, the key could be set to a known good node such as one operated by the person generating the torrent.
    #[serde(default)]
    pub nodes: Vec<Node>,
    #[serde(default)]
    #[serde(rename = "creation date")]
    pub creation_date: Option<u64>,
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    #[serde(default)]
    #[serde(rename = "created by")]
    pub created_by: Option<String>,

    #[serde(default)]
    pub encoding: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Info {
    /// The name key maps to a UTF-8 encoded string which is the suggested name
    /// to save the file (or directory) as. It is purely advisory.
    pub name: String,
    /// This key is only present if no files key is present. One of the two must be present.
    /// If “length” is present then the download represents a single file.
    /// Length maps to the length of the file in bytes.
    ///
    /// For the purposes of the other keys, the multi-file case is treated as only having
    /// a single file by concatenating the files in the order they appear in the files list.
    /// The files list is the value files maps to, and is a list of dictionaries containing the following keys:
    #[serde(default)]
    pub length: Option<i64>,
    /// The **piece length** maps to the number of bytes in each piece the file is split into.
    /// For the purposes of transfer, files are split into fixed-size pieces which are all
    /// the same length, except for possibly the last one which may be truncated.
    /// piece length is almost always a power of two.
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    /// **pieces** maps to a string whose length is a multiple of 20. It is to be subdivided into
    /// strings of length 20, each of which is the SHA1 hash of the piece at the corresponding index.
    #[serde(with = "serde_bytes")]
    pub pieces: Vec<u8>,
    #[serde(default)]
    /// This key is only present if no length key is present. One of the two must be present.
    /// If files is present the metafile represent a set of files which go in a directory structure.
    /// files maps to a list representing all files in to metafile.
    pub files: Option<Vec<File>>,

    /// Private torrents
    ///
    /// Private torrents are indicated by the key-value pair “private: 1” in the “info” dict
    /// of the torrent’s metainfo file. This is used to disable peer sharing mechanism such as DHT and PEX.
    #[serde(default)]
    pub private: Option<u8>,
    #[serde(default)]
    pub md5sum: Option<String>,
    #[serde(default)]
    pub path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "root hash")]
    pub root_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Node(pub String, pub i64);

/// Each file maps to dictionaries containing two keys :
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct File {
    /// The length of the file, in bytes.
    pub length: u64,
    #[serde(default)]
    pub md5sum: Option<String>,
    /// A list of UTF-9 encoded strings corresponding to subdirectory names, the last
    /// of which is the actual file name (a zero length list is an error case).
    pub path: Vec<String>,
}
