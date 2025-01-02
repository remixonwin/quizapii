FROM mcr.microsoft.com/vscode/devcontainers/rust:latest

# Install Node.js, npm, and Docker prerequisites
RUN apt-get update && apt-get install -y \
    nodejs \
    npm \
    apt-transport-https \
    ca-certificates \
    curl \
    gnupg \
    lsb-release

# Install Docker's official GPG key and repository
RUN curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg \
    && echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian \
    $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null

# Install Docker CLI and ensure docker group exists
RUN apt-get update && apt-get install -y docker-ce-cli \
    && (getent group docker || groupadd docker)

# Setup Docker permissions properly
RUN usermod -aG docker vscode

# Set proper workspace permissions
WORKDIR /workspace
RUN chown vscode:vscode /workspace

# Copy package.json with correct ownership and install project dependencies
COPY --chown=vscode:vscode package.json /workspace/
RUN npm install

# Copy project files with correct ownership
COPY --chown=vscode:vscode . /workspace

# Switch to non-root user
USER vscode
