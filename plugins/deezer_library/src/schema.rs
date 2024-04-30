// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use neko_sqlx::*;

schema! {
  pub enum DeezerLibraryTracks {
    TrackId.big_integer().primary_key(),
    AlbumId.big_integer().not_null(),
    ArtistId.big_integer().not_null(),
    Available.boolean().not_null();

    Self
      .foreign_key(fk!(DeezerLibraryAlbums, AlbumId, Cascade, Cascade))
      .foreign_key(fk!(DeezerLibraryArtists, ArtistId, Cascade, Cascade))
  }
  pub enum DeezerLibraryAlbums {
    AlbumId.big_integer().primary_key(),
    ArtistId.big_integer().not_null();

    Self
      .foreign_key(fk!(DeezerLibraryArtists, ArtistId, Cascade, Cascade))
  }
  pub enum DeezerLibraryArtists {
    ArtistId.big_integer().primary_key()
  }
}
