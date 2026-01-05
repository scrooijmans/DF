# MudRock Development Setup Guide

## 🎯 **Overview**

This guide covers the complete development environment setup for MudRock, including **Docker-based development**, **CI/CD pipeline**, and **native distribution** approaches.

## 🚀 **Quick Start**

### **Prerequisites**
- **Docker Desktop**: [Download here](https://www.docker.com/products/docker-desktop/)
- **Node.js**: v18+ for frontend development
- **Rust**: Latest stable for backend development

### **Development Workflow**
```bash
# 1. Clone the repository
git clone https://github.com/your-org/mudrock.git
cd mudrock

# 2. Start development services
docker-compose -f docker-compose.dev.yml up -d

# 3. Install frontend dependencies
npm install

# 4. Start development server
cargo tauri dev
```

## 🔧 **Development Environment Options**

### **Option 1: Docker Development (RECOMMENDED)**

#### **Benefits**
- ✅ **Consistent Environment**: Same PostgreSQL/Qdrant versions for all developers
- ✅ **Easy Setup**: `docker-compose up` and everything is ready
- ✅ **Isolated Development**: No conflicts with local PostgreSQL/Qdrant installations
- ✅ **CI/CD Ready**: Same environment in GitHub Actions

#### **Setup**
```bash
# Start development services
docker-compose -f docker-compose.dev.yml up -d

# Check service status
docker-compose -f docker-compose.dev.yml ps

# View logs
docker-compose -f docker-compose.dev.yml logs -f

# Stop services
docker-compose -f docker-compose.dev.yml down

# Reset data (careful!)
docker-compose -f docker-compose.dev.yml down -v
docker-compose -f docker-compose.dev.yml up -d
```

### **Option 2: Native Development**

#### **Prerequisites**
- **PostgreSQL 17.5**: Install locally
- **Qdrant**: Download and run locally
- **Node.js**: v18+ for frontend development
- **Rust**: Latest stable for backend development

#### **Setup Steps**
```bash
# 1. Install PostgreSQL locally
# Download from: https://www.postgresql.org/download/

# 2. Install Qdrant locally
# Download from: https://github.com/qdrant/qdrant/releases

# 3. Start services manually
# PostgreSQL: Start service or run postgres
# Qdrant: Run qdrant executable

# 4. Install frontend dependencies
npm install

# 5. Start development server
cargo tauri dev
```

## 📊 **Development vs Production**

### **Development Environment**
- ✅ **Docker Services**: PostgreSQL and Qdrant in containers
- ✅ **Hot Reload**: Frontend changes reflect immediately
- ✅ **Debug Mode**: Detailed logging and error messages
- ✅ **Test Data**: Sample data for development

### **Production Distribution**
- ✅ **Native Installers**: MSI and NSIS installers
- ✅ **Smart Binary Downloader**: Downloads PostgreSQL/Qdrant on first launch
- ✅ **Optimized Build**: Release mode with minimal logging
- ✅ **User Data**: Real user data and configurations

## 🚀 **Build Process**

### **Development Build**
```bash
# Development mode (with hot reload)
cargo tauri dev
```

### **Production Build**
```bash
# Build for production
cargo tauri build

# Creates installers in src-tauri/target/release/
# - MudRock_1.0.0_x64_en-US.msi
# - MudRock_1.0.0_x64-setup.exe
```

## 🔍 **Debugging**

### **Service Health Checks**
```bash
# Check PostgreSQL
curl http://localhost:5432

# Check Qdrant
curl http://localhost:6333/

# Check Docker services
docker-compose -f docker-compose.dev.yml ps
```

### **Logs**
```bash
# Application logs
tail -f logs/mudrock.log

# Docker service logs
docker-compose -f docker-compose.dev.yml logs -f postgresql
docker-compose -f docker-compose.dev.yml logs -f qdrant

# Tauri logs
cargo tauri dev -- --log-level debug
```

### **Database Access**
```bash
# Connect to PostgreSQL
psql -h localhost -U postgres -d mudrock

# Or via Docker
docker exec -it mudrock-postgresql-dev psql -U postgres -d mudrock
```

## 🧪 **Testing**

### **Unit Tests**
```bash
# Run Rust tests
cargo test

# Run frontend tests
npm test

# Run all tests
npm run test:all
```

### **Integration Tests**
```bash
# Start test services
docker-compose -f docker-compose.dev.yml up -d

# Run integration tests
cargo test --test integration

# Clean up
docker-compose -f docker-compose.dev.yml down
```

### **End-to-End Tests**
```bash
# Build application
cargo tauri build

# Run E2E tests
npm run test:e2e
```

## 🎯 **Benefits of Docker in Development**

### **✅ Why Use Docker for Development?**

#### **1. Consistent Environment**
- **Same Versions**: All developers use PostgreSQL 17.5 and Qdrant 1.14.1
- **No Conflicts**: Doesn't interfere with local PostgreSQL installations
- **Reproducible**: Same setup everywhere, every time

#### **2. Easy Onboarding**
- **New Developers**: Just run `docker-compose up -d`
- **No Manual Setup**: No need to install PostgreSQL/Qdrant locally
- **Quick Reset**: `docker-compose down -v` to start fresh

#### **3. CI/CD Ready**
- **Same Environment**: Development matches CI/CD pipeline
- **Automated Testing**: Services available in GitHub Actions
- **Consistent Results**: Tests run in same environment

#### **4. Isolated Development**
- **No System Pollution**: Doesn't affect local PostgreSQL
- **Multiple Projects**: Can run different versions simultaneously
- **Clean Slate**: Easy to reset and start over

### **❌ Why NOT Use Docker for End Users?**

#### **1. User Experience**
- **Complex Setup**: Users need Docker knowledge
- **Larger Footprint**: ~200MB Docker overhead
- **Performance Impact**: Container overhead vs native

#### **2. Industry Standards**
- **Desktop Apps**: Usually native installers (VS Code, PostgreSQL)
- **User Expectations**: Simple installer experience
- **No Dependencies**: Users don't need technical knowledge

## 🎯 **Development Workflow Clarification**

### **✅ Do We Still Use `cargo tauri dev`?**

**YES!** We still use `cargo tauri dev` for development. Here's the workflow:

#### **Development Workflow**
```bash
# 1. Start Docker services (background)
docker-compose -f docker-compose.dev.yml up -d

# 2. Start Tauri development (foreground)
cargo tauri dev
```

#### **What This Gives Us**
- ✅ **Hot Reload**: Frontend changes reflect immediately
- ✅ **Debug Mode**: Detailed logging and error messages
- ✅ **Service Connections**: Tauri connects to Docker PostgreSQL/Qdrant
- ✅ **Consistent Environment**: Same services for all developers

### **🔄 Two Development Modes**

#### **Mode 1: Docker Development (RECOMMENDED)**
```bash
# Start services
docker-compose -f docker-compose.dev.yml up -d

# Start development
cargo tauri dev
```

#### **Mode 2: Native Development**
```bash
# Install PostgreSQL/Qdrant locally
# Start services manually

# Start development
cargo tauri dev
```

## 🎯 **Best Practices**

### **Development**
1. **Use Docker for Development**: Consistent environment across team
2. **Keep Services Running**: Use `docker-compose up -d` to start services
3. **Monitor Logs**: Check service logs for issues
4. **Test Regularly**: Run tests before committing
5. **Use Environment Variables**: For configuration flexibility

### **Code Quality**
1. **Follow Rust Conventions**: Use `cargo fmt` and `cargo clippy`
2. **Follow Svelte Conventions**: Use `npm run lint` and `npm run format`
3. **Write Tests**: Unit tests for critical functionality
4. **Document Changes**: Update documentation for new features

### **Performance**
1. **Optimize Builds**: Use release mode for production
2. **Monitor Resources**: Check memory and CPU usage
3. **Profile Code**: Use profiling tools for bottlenecks
4. **Cache Dependencies**: Use Docker layer caching

## 🚀 **Next Steps**

### **Immediate**
1. **Set up Docker development environment**
2. **Test with containerized services**
3. **Update documentation**
4. **Create team onboarding guide**

### **Short Term**
1. **CI/CD integration with Docker**
2. **Automated testing pipeline**
3. **Performance monitoring**
4. **Security scanning**

### **Long Term**
1. **Evaluate production Docker deployment**
2. **Multi-platform support**
3. **Enterprise features**
4. **Cloud deployment options**

## 🎯 **Conclusion**

**Docker is excellent for development** because it provides:
- ✅ **Consistent Environment**: Same setup for all developers
- ✅ **Easy Onboarding**: New developers just run `docker-compose up`
- ✅ **Isolated Services**: No conflicts with local installations
- ✅ **CI/CD Ready**: Same environment in automated testing

**Native distribution is better for end users** because it provides:
- ✅ **Simple Installation**: Users just run the installer
- ✅ **No Dependencies**: No Docker knowledge required
- ✅ **Better Performance**: Native binaries vs container overhead
- ✅ **Professional UX**: Industry-standard installation experience

**We still use `cargo tauri dev`** for development, but now with Docker services providing consistent PostgreSQL and Qdrant instances.

This hybrid approach gives us the best of both worlds! 🚀 