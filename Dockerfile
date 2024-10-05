# Use the official Rust image as a parent image
FROM rust:1.74 as builder

# Set the working directory in the container
WORKDIR /usr/src/app

# Install dependencies
RUN apt-get update && apt-get install -y \
   pkg-config \
   libssl-dev \
   nodejs \
   npm

# Install trunk and add wasm32-unknown-unknown target
RUN cargo install trunk && \
   rustup target add wasm32-unknown-unknown

# Copy the entire project
COPY . .

# Build the application
RUN trunk build --release

# Use a lightweight server to serve the static files
FROM nginx:alpine

# Copy the built files from the builder stage
COPY --from=builder /usr/src/app/dist /usr/share/nginx/html

# Copy a custom nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Expose port 80
EXPOSE 80

# Start Nginx
CMD ["nginx", "-g", "daemon off;"]