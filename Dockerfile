FROM ubuntu:22.04@sha256:bcc511d82482900604524a8e8d64bf4c53b2461868dac55f4d04d660e61983cb

RUN apt-get update
RUN apt-get install -y --no-install-recommends ca-certificates clang curl libssl-dev pkg-config git dialog
RUN groupadd --gid 1000 appuser
RUN useradd -s /bin/bash -u 1000 -g appuser -m appuser
RUN apt-get install -y sudo
RUN echo appuser ALL=\(ALL\) NOPASSWD: ALL >> /etc/sudoers.d/appuser
RUN chmod 0440 /etc/sudoers.d/appuser
USER appuser

RUN curl --proto '=https' --tlsv1.2 --retry 10 --retry-connrefused -fsSL 'https://sh.rustup.rs' | sh -s -- -y
ENV PATH="/home/appuser/.cargo/bin:${PATH}"
RUN curl -L https://sp1.succinct.xyz | bash && ~/.sp1/bin/sp1up

WORKDIR /home/appuser/program

ENV CARGO_TERM_COLOR=always
ENTRYPOINT ["/bin/bash"]
# ENTRYPOINT [ "/root/.sp1/bin/cargo-prove" ]
# CMD [ "prove" ]
