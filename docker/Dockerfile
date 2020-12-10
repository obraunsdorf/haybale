FROM ubuntu:18.04

RUN apt-get update && apt-get install -y \
    llvm-10 \
    build-essential \
    cmake \
    curl \
    git \
    zlib1g-dev

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

WORKDIR /boolector
# Download and build Boolector
RUN git clone https://github.com/boolector/boolector .
# Download and build Lingeling
RUN ./contrib/setup-lingeling.sh

# Download and build BTOR2Tools
RUN ./contrib/setup-btor2tools.sh

# Build Boolector
RUN ./configure.sh --shared && cd build && make -j$(nproc)

# Install Boolector
RUN make install

WORKDIR /haybale
