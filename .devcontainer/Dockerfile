FROM rust:latest

# ユーザー作成 
ARG USER_NAME=utakata
ARG USER_ID=1000
ARG GROUP_ID=1000

RUN groupadd -g ${GROUP_ID} ${USER_NAME} && \
    useradd -m -u ${USER_ID} -g ${USER_NAME} ${USER_NAME}

# Gitの導入
RUN apt update -y \
    && apt upgrade -y \
    && apt install git curl -y \ 
    && apt clean


RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh \
    && rustup target add wasm32-unknown-unknown \
    && cargo install --locked trunk \
    && cargo +nightly install miniserve

# デフォルトシェルをbashに設定
RUN chsh -s /bin/bash ${USER_NAME}
RUN chown -R ${USER_NAME}:${USER_NAME} /usr/local/cargo/registry && \
    chown -R ${USER_NAME}:${USER_NAME} /usr/local/cargo/
USER ${USER_NAME}