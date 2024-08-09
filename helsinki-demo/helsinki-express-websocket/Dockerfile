# Use the official Node.js image
FROM node:14

# Set working directory
WORKDIR /app

# Copy package.json and package-lock.json
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy source files
COPY . .

# Build the TypeScript code
RUN npm run build

# Expose the port your server runs on
EXPOSE 3000 

# Start the application
CMD ["npm", "start"]

