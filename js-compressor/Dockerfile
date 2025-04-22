# Use official Node.js base image
FROM node:20-slim

# Set working directory
WORKDIR /app

# Copy package.json and install dependencies
COPY package.json package-lock.json* ./
RUN npm install

# Copy source files
COPY . .

# Use node CLI to run
ENTRYPOINT ["node", "index.js"]
