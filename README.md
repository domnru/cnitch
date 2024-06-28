# Cnitch - ALPHA DEVELOPMENT

[![Build Status](https://img.shields.io/badge/build-unkown-red)](https://github.com/domnru/cnitch) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

cnitch (Command Snitch) is a security tool designed to monitor unauthorized access attempts on critical system binaries. It replaces binaries in `/usr/bin` with harmless placeholders that snitch the command to the cnitch server container.
**Use it carefully. Its not production ready and not enough tested.**

## How It Works

1. **Identification:** You specify which binaries in `/usr/bin` should be replaced by the cnitch client.
2. **Replacement:** The build.sh replaces these binaries with the cnitch client by providing an interactive simple dialog.
3. **Alerting:** If an attacker tries to execute one of the monitored programs, a alert triggered which will send metadata like which binary was executed and which params where used.
4. **Logging:** All execution attempts are logged inside the cnitch container with the alerted metadata and a timestamp in UTC.

## Installation

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/domnru/cnitch.git
   ```
   Enter the project:
   ```bash
   cd cnitch
   ```

3. **Build an start the server:**
   From the root folder of the project:
   ```bash
   docker compose build
   ```
   By default this will build the Dockerfile in:
   ```
   cnitch/server/x86.Dockerfile
   ```
   The builded Image is less than 500KB in size. The server image is a Rust Binary without any dependencies. Its early stage and im working to improve it. 
   
   Lets start the server container:
   1. Set the `SECRET` environment variable in the `compose.yml`
   2. Set a port for the backend to listen to via the `PORT` environment variable (Optional). Defaults to 8080.
   3. Start the container     
   ```bash
   docker compose up
   ```

5. **Build the client:**
   From the root folder of the project:
   ```bash
   cd client
   build_client.sh
   ```
   This will run the `docker build` with `client/Dockerfile`. Then a container will be started with the client binary. After that the binary is in the root folder of the project `cnitch-client`.

6. **Swap the binaries**
   Set the `CNITCH_HOST` environment variable inside the containers you want to monitor.
   Don't forget the correct "Docker Networking" so the monitored containers can actually access the cnitch container under the `CNITCH_HOST` variable.
   In the root folder of the repository is a script called `build.sh`. Run it:
   ```bash
   ./build.sh
   ```
   This will run a "dialog" inside the console asking you which containers you want to change and then which binaries inside them.

8. **Testing it**
   Mock an attacker executing commands inside the attacked container. To start an interactive sh session inside the container:
   ```bash
   docker exec -it [CONTAINER_ID] /bin/sh
   ```
   Now run any command you selected inside step 6.
   ```bash
   apt install my_shady_software -y
   ```
   Nothing should happen inside the console.

9. **Reading the monitored data**
   You have two options to read the data.
   1. Via volumes:
      Create the data.json inside the root folder of the repository.
      Create the volume: `./data.json:/app/data` inside the `compose.yml`
      After restarting your container you should see that the data.json fills with human readable data after you redo step 8.
   2. Via Networking:
      If you set the `SECRET` environment variable inside the cnitch container you have the ability to send a http POST request to the container. The body should contain the secret via text/plain. After that you will get the content of the data file returned.
      **Example with curl**
      ```bash
      curl -X POST -H "Content-Type: text/plain" -d "${SECRET}" http://${CNITCH_HOST}

      
      
   
