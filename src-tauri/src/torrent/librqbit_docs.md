# librqbit docs

## Crate

### Structs

```rust
pub struct AddTorrentOptions {
    pub paused: bool,
    pub only_files_regex: Option<String>,
    pub only_files: Option<Vec<usize>>,
    pub overwrite: bool,
    pub list_only: bool,
    pub output_folder: Option<String>,
    pub sub_folder: Option<String>,
    pub peer_opts: Option<PeerConnectionOptions>,
    pub force_tracker_interval: Option<Duration>,
    pub disable_trackers: bool,
    pub ratelimits: LimitsConfig,
    pub initial_peers: Option<Vec<SocketAddr>>,
    pub preferred_id: Option<usize>,
    pub storage_factory: Option<BoxStorageFactory>,
    pub defer_writes: Option<bool>,
    pub trackers: Option<Vec<String>>,
}
```

```rust
pub struct ApiError { /* private fields */ }
```

* **Methods:**

  ```rust
  pub fn new_from_anyhow(status: StatusCode, error: Error) -> Self
  ```

  ```rust
  pub const fn torrent_not_found(torrent_id: TorrentIdOrHash) -> Self
  ```

  ```rust
  pub const fn new_from_text(status: StatusCode, text: &'static str) -> Self
  ```

  ```rust
  pub fn not_implemented(msg: &str) -> Self
  ```

  ```rust
  pub const fn dht_disabled() -> Self
  ```

  ```rust
  pub const fn unathorized() -> Self
  ```

  ```rust
  pub fn status(&self) -> StatusCode
  ```

  ```rust
  pub fn with_status(self, status: StatusCode) -> Self
  ```

  ```rust
  pub fn with_plaintext_error(self, value: bool) -> Self
  ```

```rust
pub struct AzureusStyle {
    pub kind: AzureusStyleKind,
    pub version: [u8; 4],
}
```

```rust
pub struct ByteBuf<'a>(pub &'a [u8]);
```

```rust
pub struct ByteBufOwned(pub Bytes);
```

```rust
pub struct CreateTorrentOptions<'a> {
    pub name: Option<&'a str>,
    pub piece_length: Option<u32>,
}
```

```rust
pub struct FileDetails<'a, BufType> {
    pub filename: FileIteratorName<'a, BufType>,
    pub len: u64,
    pub sha1: Option<&'a BufType>,
    pub symlink_path: Option<[BufType]>,
    /* private fields */
}
```

* **Methods:**

  ```rust
  pub fn attrs(&self) -> FileDetailsAttrs
  ```

```rust
pub struct FileDetailsAttrs {
    pub executable: bool,
    pub hidden: bool,
    pub padding: bool,
    pub symlink: bool,
}
```

```rust
pub struct FileDetailsExt<'a, BufType> {
    pub details: FileDetails<'a, BufType>,
    pub offset: u64,
    pub pieces: Range<u32>,
}
```

* **Methods:**

  ```rust
  pub fn pieces_usize(&self) -> Range<usize>
  ```

```rust
pub struct ListOnlyResponse {
    pub info_hash: Id20,
    pub info: TorrentMetaV1Info<ByteBufOwned>,
    pub only_files: Option<Vec<usize>>,
    pub output_folder: PathBuf,
    pub seen_peers: Vec<SocketAddr>,
    pub torrent_bytes: Bytes,
}
```

```rust
pub struct Magnet {
    pub trackers: Vec<String>,
    /* private fields */
}
```

* **Methods:**

  ```rust
  pub fn as_id20(&self) -> Option<Id<20>>
  ```

  ```rust
  pub fn as_id32(&self) -> Option<Id<32>>
  ```

  ```rust
  pub fn from_id20(id20: Id<20>, trackers: Vec<String>) -> Magnet
  ```

  ```rust
  pub fn parse(url: &str) -> Result<Magnet, Error>
  ```

```rust
pub struct ManagedTorrent {
    pub shared: Arc<ManagedTorrentShared>,
    pub metadata: ArcSwapOption<TorrentMetadata>,
    /* private fields */
}
```

* **Methods:**

  ```rust
  pub fn stream(self: Arc<Self>, file_id: usize) -> Result<FileStream>
  ```

  ```rust
  pub fn id(&self) -> usize
  ```

  ```rust
  pub fn name(&self) -> Option<String>
  ```

  ```rust
  pub fn shared(&self) -> &ManagedTorrentShared
  ```

  ```rust
  pub fn with_metadata<R>(&self, f: impl FnMut(&Arc<TorrentMetadata>) -> R) -> Result<R>
  ```

  ```rust
  pub fn info_hash(&self) -> Id20
  ```

  ```rust
  pub fn only_files(&self) -> Option<Vec<usize>>
  ```

  ```rust
  pub fn with_state<R>(&self, f: impl FnOnce(&ManagedTorrentState) -> R) -> R
  ```

  ```rust
  pub fn live(&self) -> Option<Arc<TorrentStateLive>>
  ```

  ```rust
  pub fn is_paused(&self) -> bool
  ```

  ```rust
  pub fn stats(&self) -> TorrentStats
  ```

  ```rust
  pub fn wait_until_initialized(&self) -> BoxFuture<'_, Result<()>>
  ```

  ```rust
  pub fn wait_until_completed(&self) -> BoxFuture<'_, Result<()>>
  ```

```rust
pub struct ManagedTorrentShared {
    pub id: usize,
    pub info: TorrentMetaV1Info<ByteBufOwned>,
    pub torrent_bytes: Bytes,
    pub info_bytes: Bytes,
    pub info_hash: Id20,
    pub trackers: HashSet<String>,
    pub peer_id: Id20,
    pub lengths: Lengths,
    pub file_infos: FileInfos,
    pub span: Span,
    /* private fields */
}
```

```rust
pub struct ParsedTorrent<BufType> {
    pub meta: TorrentMetaV1<BufType>,
    pub info_bytes: BufType,
}
```

```rust
pub struct PeerConnectionOptions {
    pub connect_timeout: Option<Duration>,
    pub read_write_timeout: Option<Duration>,
    pub keep_alive_interval: Option<Duration>,
}
```

```rust
pub struct Session {
    pub ratelimits: Limits,
    pub blocklist: Blocklist,
    /* private fields */
}
```

* **Methods:**

  ```rust
  pub fn new(default_output_folder: PathBuf) -> BoxFuture<'static, Result<Arc<Self>>>
  ```

  ```rust
  pub fn cancellation_token(&self) -> &CancellationToken
  ```

  ```rust
  pub fn new_with_opts(default_output_folder: PathBuf, opts: SessionOptions) -> BoxFuture<'static, Result<Arc<Self>>>
  ```

  ```rust
  pub fn get_dht(&self) -> Option<&Dht>
  ```

  ```rust
  pub fn spawn(&self, span: Span, fut: impl Future<Output = Result<()>> + Send + 'static)
  ```

  ```rust
  pub async fn stop(&self)
  ```

  ```rust
  pub fn with_torrents<R>(&self, callback: impl Fn(&mut dyn Iterator<Item = (usize, &Arc<ManagedTorrent>)>) -> R) -> R
  ```

  ```rust
  pub fn add_torrent<'a>(&'a self, add: AddTorrent<'a>, opts: Option<AddTorrentOptions>) -> BoxFuture<'a, Result<AddTorrentResponse>>
  ```

  ```rust
  pub fn get(&self, id: TorrentIdOrHash) -> Option<Arc<ManagedTorrent>>
  ```

  ```rust
  pub async fn delete(&self, id: TorrentIdOrHash, delete_files: bool) -> Result<()>
  ```

  ```rust
  pub fn make_peer_rx_managed_torrent(&self, t: &Arc<ManagedTorrent>, announce: bool) -> Option<BoxStream<'static, SocketAddr>>
  ```

  ```rust
  pub async fn pause(&self, handle: &Arc<ManagedTorrent>) -> Result<()>
  ```

  ```rust
  pub async fn unpause(&self, handle: &Arc<ManagedTorrent>) -> Result<()>
  ```

  ```rust
  pub async fn update_only_files(&self, handle: &Arc<ManagedTorrent>, only_files: &HashSet<usize>) -> Result<()>
  ```

  ```rust
  pub fn tcp_listen_port(&self) -> Option<u16>
  ```

  ```rust
  pub fn stats_snapshot(&self) -> SessionStatsSnapshot
  ```

```rust
pub struct SessionOptions {
    pub disable_dht: bool,
    pub disable_dht_persistence: bool,
    pub dht_config: Option<PersistentDhtConfig>,
    pub fastresume: bool,
    pub cancellation_token: Option<CancellationToken>,
    pub concurrent_init_limit: Option<usize>,
    pub root_span: Option<Span>,
    pub default_storage_factory: Option<BoxStorageFactory>,
    pub defer_writes_up_to: Option<usize>,
    pub socks_proxy_url: Option<String>,
    pub peer_id: Option<Id20>,
    pub peer_opts: Option<PeerConnectionOptions>,
    pub listen_port_range: Option<Range<u16>>,
    pub persistence: Option<SessionPersistenceConfig>,
    pub enable_upnp_port_forwarding: bool,
}
```

```rust
pub struct TorrentMetaV1<BufType> {
    pub announce: Option<BufType>,
    pub announce_list: Vec<Vec<BufType>>,
    pub info: TorrentMetaV1Info<BufType>,
    pub comment: Option<BufType>,
    pub created_by: Option<BufType>,
    pub encoding: Option<BufType>,
    pub publisher: Option<BufType>,
    pub publisher_url: Option<BufType>,
    pub creation_date: Option<usize>,
    pub info_hash: Id<20>,
}
```

* **Methods:**

  ```rust
  pub fn iter_announce(&self) -> impl Iterator<Item = &BufType>
  ```

```rust
pub struct TorrentMetaV1File<BufType> {
    pub length: u64,
    pub path: Vec<BufType>,
    pub attr: Option<BufType>,
    pub sha1: Option<BufType>,
    pub symlink_path: Option<Vec<BufType>>,
}
```

* **Methods:**

  ```rust
  pub fn full_path(&self, parent: &mut PathBuf) -> Result<(), Error>
  ```

```rust
pub struct TorrentMetaV1Info<BufType> {
    pub name: Option<BufType>,
    pub pieces: BufType,
    pub piece_length: u32,
    pub length: Option<u64>,
    pub md5sum: Option<BufType>,
    pub files: Option<Vec<TorrentMetaV1File<BufType>>>,
}
```

* **Methods:**

  ```rust
  pub fn compare_hash(&self, piece: u32, hash: [u8; 20]) -> Option<bool>
  ```

  ```rust
  pub fn get_hash(&self, piece: u32) -> Option<&[u8]>
  ```

  ```rust
  pub fn iter_file_details<'a>(&'a self, lengths: &'a Lengths) -> Result<impl Iterator<Item = FileDetails<'a, BufType>> + 'a, Error>
  ```

  ```rust
  pub fn iter_file_lengths(&self) -> Result<impl Iterator<Item = u64>, Error>
  ```

  ```rust
  pub fn iter_filenames_and_lengths(&self) -> Result<impl Iterator<Item = (FileIteratorName<'_, BufType>, u64)>, Error>
  ```

```rust
pub struct TorrentMetadata {
    pub info: TorrentMetaV1Info<ByteBufOwned>,
    pub torrent_bytes: Bytes,
    pub info_bytes: Bytes,
    pub lengths: Lengths,
    pub file_infos: FileInfos,
    pub name: Option<String>,
}
```

```rust
pub struct TorrentStats {
    pub state: TorrentStatsState,
    pub file_progress: Vec<u64>,
    pub error: Option<String>,
    pub progress_bytes: u64,
    pub uploaded_bytes: u64,
    pub total_bytes: u64,
    pub finished: bool,
    pub live: Option<LiveStats>,
}
```

* **Methods:**

  ```rust
  pub fn progress_percent_human_readable(&self) -> impl Display
  ```

  ```rust
  pub fn progress_bytes_human_readable(&self) -> impl Display
  ```

### Enums

```rust
pub enum AddTorrent<'a> {
    Url(Cow<'a, str>),
    TorrentFileBytes(Cow<'a, [u8]>),
    TorrentInfo(Box<TorrentMetaV1Owned>),
}
```

```rust
pub enum AddTorrentResponse {
    AlreadyManaged(usize, Arc<ManagedTorrent>),
    ListOnly(ListOnlyResponse),
    Added(usize, Arc<ManagedTorrent>),
}
```

* **Methods:**

  ```rust
  pub fn into_handle(self) -> Option<Arc<ManagedTorrent>>
  ```

```rust
pub enum AzureusStyleKind {
    Deluge,
    LibTorrent,
    Other,
    QBittorrent,
    RQBit,
    Transmission,
    UTorrent,
}
```

* **Methods:**

  ```rust
  pub fn from_bytes(bytes: impl Into<Bytes>) -> Self
  ```

```rust
pub enum FileIteratorName<'a, BufType> {
    Single(Option<&'a BufType>),
    Tree(&'a [BufType]),
}
```

* **Methods:**

  ```rust
  pub fn iter_components(&self) -> impl Iterator<Item = Result<&'a str, Error>>
  ```

  ```rust
  pub fn to_pathbuf(&self) -> Result<PathBuf, Error>
  ```

  ```rust
  pub fn to_string(&self) -> Result<String, Error>
  ```

  ```rust
  pub fn to_vec(&self) -> Result<Vec<String>, Error>
  ```

```rust
pub enum ManagedTorrentState {
    Initializing(Arc<TorrentStateInitializing>),
    Paused(TorrentStatePaused),
    Live(Arc<TorrentStateLive>),
    Error(Error),
    None,
}
```

* **Methods:**

  ```rust
  pub fn name(&self) -> &'static str
  ```

```rust
pub enum PeerId { … }
```

```rust
pub enum SessionPersistenceConfig { … }
```

```rust
pub enum TorrentStatsState { … }
```

### Traits

```rust
pub trait ByteBufT { … }
```

```rust
pub trait CloneToOwned { … }
```

### Constants

```rust
pub const SUPPORTED_SCHEMES: &[&str] = …;
```

### Functions

```rust
pub fn client_name_and_version() -> String
```

```rust
pub fn create_torrent() -> Result<TorrentMetaV1Owned>
```

```rust
pub fn generate_azereus_style() -> [u8; 8]
```

```rust
pub fn generate_peer_id() -> [u8; 20]
```

```rust
pub fn librqbit_spawn<F>(fut: F) -> JoinHandle<()> 
where F: Future<Output = ()> + Send + 'static
```

```rust
pub fn torrent_from_bytes(bytes: &[u8]) -> Result<TorrentMetaV1Owned>
```

```rust
pub fn torrent_from_bytes_ext(bytes: &[u8]) -> Result<TorrentMetadata>
```

```rust
pub fn try_decode_peer_id(peer_id_bytes: &[u8]) -> Option<PeerId>
```

```rust
pub fn try_increase_nofile_limit() -> Result<()>
```

```rust
pub fn version() -> &'static str
```

### Type Aliases

```rust
pub type FileInfos = Vec<FileInfo>;
```

```rust
pub type TorrentMetaV1Borrowed<'a> = TorrentMetaV1<&'a [u8]>;
```

```rust
pub type TorrentMetaV1Owned = TorrentMetaV1<ByteBufOwned>;
```

## api

### Structs

```rust
pub struct Api { /* private fields */ }
```

```rust
pub struct ApiAddTorrentResponse {
    pub id: Option<usize>,
    pub details: TorrentDetailsResponse,
    pub output_folder: String,
    pub seen_peers: Option<Vec<SocketAddr>>,
}
```

```rust
pub struct ApiTorrentListOpts {
    pub with_stats: bool,
}
```

```rust
pub struct EmptyJsonResponse {}
```

```rust
pub struct LiveStats {
    pub snapshot: StatsSnapshot,
    pub average_piece_download_time: Option<Duration>,
    pub download_speed: Speed,
    pub upload_speed: Speed,
    pub time_remaining: Option<DurationWithHumanReadable>,
}
```

```rust
pub struct TorrentDetailsResponse {
    pub info_hash: String,
    pub name: Option<String>,
    pub files: Vec<TorrentDetailsResponseFile>,
}
```

```rust
pub struct TorrentDetailsResponseFile {
    pub name: String,
    pub components: Vec<String>,
    pub length: u64,
    pub included: bool,
    pub attributes: FileDetailsAttrs,
}
```

```rust
pub struct TorrentListResponse {
    pub torrents: Vec<TorrentDetailsResponse>,
}
```

```rust
pub struct TorrentStats {
    … /* same as crate TorrentStats */
}
```

### Enums

```rust
pub enum TorrentIdOrHash { … }
```

*(Additional module items omitted for brevity.)*

## file\_info

```rust
pub struct FileInfo {
    pub relative_filename: PathBuf,
    pub offset_in_torrent: u64,
    pub piece_range: Range<u32>,
    pub attrs: FileDetailsAttrs,
    pub len: u64,
}
```

* **Methods:**

  ```rust
  pub fn piece_range_usize(&self) -> Range<usize>
  ```

  ```rust
  pub fn iter_piece_priorities(&self) -> impl Iterator<Item = usize>
  ```

## http\_api\_client

```rust
pub struct HttpApiClient { /* private fields */ }
```

* **Methods:**

  ```rust
  pub fn new(url: &str) -> Result<Self>
  ```

  ```rust
  pub fn base_url(&self) -> &Url
  ```

  ```rust
  pub fn validate_rqbit_server(&self) -> BoxFuture<'_, Result<()>>
  ```

  ```rust
  pub fn add_torrent<'a>(&'a self, torrent: AddTorrent<'a>, opts: Option<AddTorrentOptions>) -> BoxFuture<'a, Result<ApiAddTorrentResponse>>
  ```

## http\_api\_types

```rust
pub struct InitialPeers(pub Vec<SocketAddr>);
```

```rust
pub struct OnlyFiles(pub Vec<usize>);
```

```rust
pub struct TorrentAddQueryParams {
    pub torrent: Option<String>,
    pub magnet: Option<String>,
    pub path: Option<String>,
    pub filename: Option<String>,
    pub fast_resume_path: Option<String>,
    pub download_path: Option<String>,
}
```

## limits

```rust
pub struct Limits {
    pub download_bps: Option<u64>,
    pub upload_bps: Option<u64>,
}
```

```rust
pub struct LimitsConfig {
    pub download_bps: Option<u64>,
    pub upload_bps: Option<u64>,
}
```

## session\_stats

```rust
pub struct SessionStats {
    pub torrents: usize,
    pub downloading: usize,
    pub seeding: usize,
    pub completed: usize,
}
```

```rust
pub mod atomic {
    pub struct AtomicSessionStats { … }
}
```

```rust
pub mod snapshot {
    pub struct SessionStatsSnapshot { … }
}
```

## storage

```rust
pub struct BoxStorageFactory(Box<dyn TorrentStorage>);
```

```rust
pub trait StorageFactory { … }
```

```rust
pub trait StorageFactoryExt { … }
```

```rust
pub trait TorrentStorage { … }
```

```rust
pub mod filesystem {
    pub struct FilesystemStorage { … }
    pub struct FilesystemStorageFactory { … }
    pub struct MmapFilesystemStorage { … }
    pub struct MmapFilesystemStorageFactory { … }
}
```
