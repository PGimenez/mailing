# Use a small, Alpine-based image as the runtime base
# You might need to match the version with your build environment or use a slim version of the same base image
FROM rust:latest

# Create an app directory to hold the application
WORKDIR /app

# Copy the build executable from the local build directory into the image
# Adjust the source path to match where the built binary is located relative to the Dockerfile
# This assumes that the binary is named 'mailing' and is located in the 'target/release/' directory
COPY mailing/target/release/mailing /app/

# If your application requires additional resources, configurations, or libraries, copy them here
# COPY other resources as needed

# The default command to run when the container starts
# Adjust if your binary expects arguments or if you use a different binary name
CMD ["./mailing"]
