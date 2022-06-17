# VRS Documentation

## Installation

### From source

1. Clone the repository change your directory;

```
git clone https://github.com/PeterPierinakos/vanilla-rustlang-server
cd vanilla-rustlang-server
```

2. Start the compilation process;

```
make build
```

3. Copy the HTML and run the program:

```
# (Root may be required)
./setup.sh
```

#### Docker

```
sudo docker build -f production/docker/alpine/Dockerfile -t vrs .
```

#### Without Docker

```
# (Root may be required)
sudo ./setup.sh
sudo make run
```

Have fun!
