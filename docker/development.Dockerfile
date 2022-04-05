FROM rust:1.59.0

WORKDIR /code

ARG PACKAGES="\
  git \
  make \
  jq"

ARG USERNAME
ARG USER_UID
ARG USER_GID

RUN apt-get update && \
  apt-get install -y ${PACKAGES}

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && chown -R $USER_UID:$USER_GID /home/$USERNAME

USER ${USERNAME}

EXPOSE 3000

ENTRYPOINT ["tail", "-f", "/dev/null"]
