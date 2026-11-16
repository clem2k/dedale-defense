FROM node:22.16.0-bookworm-slim AS node
FROM rust:1.90.0-bookworm

COPY --from=node /usr/local/ /usr/local/

RUN apt-get update && apt-get install -y --no-install-recommends \
    openjdk-17-jdk-headless curl unzip zip cmake pkg-config libssl-dev \
    libasound2-dev libx11-dev libxi-dev libgl1-mesa-dev \
    && rm -rf /var/lib/apt/lists/*

ENV ANDROID_HOME=/opt/android-sdk \
    NDK_HOME=/opt/android-sdk/ndk/28.2.13676358 \
    JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64 \
    CARGO_BUILD_JOBS=2
ENV PATH="${ANDROID_HOME}/cmdline-tools/latest/bin:${ANDROID_HOME}/platform-tools:${ANDROID_HOME}/build-tools/35.0.0:${PATH}"

RUN curl --http1.1 -fsSL --retry 3 --retry-all-errors \
        https://dl.google.com/android/repository/commandlinetools-linux-13114758_latest.zip \
        -o /tmp/android-tools.zip \
    && echo "7ec965280a073311c339e571cd5de778b9975026cfcbe79f2b1cdcb1e15317ee  /tmp/android-tools.zip" | sha256sum -c - \
    && mkdir -p "$ANDROID_HOME/cmdline-tools" \
    && unzip -q /tmp/android-tools.zip -d "$ANDROID_HOME/cmdline-tools" \
    && mv "$ANDROID_HOME/cmdline-tools/cmdline-tools" "$ANDROID_HOME/cmdline-tools/latest" \
    && rm /tmp/android-tools.zip
RUN yes | sdkmanager --licenses >/dev/null
RUN sdkmanager "platform-tools" "platforms;android-35" "build-tools;35.0.0" "ndk;28.2.13676358"
RUN rustup component add rustfmt \
    && rustup target add aarch64-linux-android x86_64-linux-android \
    && cargo install --git https://github.com/not-fl3/cargo-quad-apk \
        --rev d411c8fe1c08339e46d7dc6b40ac04ea5e3a01b6 --locked

WORKDIR /workspace
CMD ["bash", "scripts/build-android.sh"]
