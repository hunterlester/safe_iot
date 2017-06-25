FROM ubuntu:trusty
RUN apt-get update && apt-get install -y \
	build-essential \
  vim \
	curl \
	file \
  -qq gcc-arm-linux-gnueabihf \
	sudo
#ENV SHELL /bin/bash
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN echo "export PATH=~/.cargo/bin:$PATH" >> ~/.bashrc
COPY . /safe_iot/
WORKDIR /safe_iot
