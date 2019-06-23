# Clipper

Clipper is a Rust binary i made for personal utils 

## Installation

``` bash
cargo build --release
```

## Usage

create the clipper config file in home directory `.clipper.conf`.
config required values:
``` toml
# -- .clipper.conf -----
# -- trello conf -------
trello_api_key = "..."
trello_api_token = "..."
trello_list_id = "..."

# -- notes conf --------
notes_file = "..."
```

``` bash
clipper --help
clipper bookmark  # save selection to trello list
clipper notes  # save selection to notes file
```

## License
[MIT](https://choosealicense.com/licenses/mit/)