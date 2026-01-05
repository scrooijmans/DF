# MudRock CI/CD Pipeline Guide

## 🎯 **Overview**

MudRock has a comprehensive CI/CD pipeline with 5 GitHub Actions workflows that cover development, testing, quality assurance, and releases.

## 📋 **Workflow Overview**

### **1. Main CI/CD Pipeline** (`ci.yml`)
**Purpose**: Primary development pipeline with Docker services

**Triggers**:
- Push to `main` or `develop` branches
- Pull requests to `main`

**What it does**:
- ✅ **Docker Services**: PostgreSQL 17.5 and Qdrant with health checks
- ✅ **Code Quality**: Rust formatting, clippy, security audit, frontend linting
- ✅ **Testing**: Rust tests, frontend tests, integration tests
- ✅ **Build**: Tauri application build
- ✅ **Artifacts**: Uploads installers for download

**Key Features**:
- Uses Docker services for consistent testing environment
- Comprehensive code quality checks
- Integration testing with real databases
- Build verification and artifact upload

### **2. Release Pipeline** (`release.yml`)
**Purpose**: Automated releases when tags are pushed

**Triggers**:
- Push tags matching `v*` pattern (e.g., `v1.0.0`)

**What it does**:
- ✅ **Release Build**: Production build with `--release` flag
- ✅ **GitHub Release**: Creates GitHub release with installers
- ✅ **Artifacts**: Uploads MSI and NSIS installers
- ✅ **Release Notes**: Auto-generates release notes

**Key Features**:
- Automated release creation
- Installer distribution
- Release notes generation
- Long-term artifact storage (90 days)

### **3. Code Quality** (`quality.yml`)
**Purpose**: Dedicated quality assurance pipeline

**Triggers**:
- Push to `main` or `develop` branches
- Pull requests to `main`

**What it does**:
- ✅ **Rust Quality**: Formatting, clippy, security audit
- ✅ **Frontend Quality**: Linting, type checking
- ✅ **Build Verification**: Compilation checks
- ✅ **Test Compilation**: Ensures tests compile

**Key Features**:
- Fast feedback on code quality
- Separate from main CI for efficiency
- Comprehensive quality gates

### **4. Rust Build & Test** (`rust.yml`)
**Purpose**: Comprehensive Rust-specific testing

**Triggers**:
- Push to `main` or `develop` branches
- Pull requests to `main`

**What it does**:
- ✅ **Docker Services**: PostgreSQL and Qdrant for testing
- ✅ **Rust Testing**: Full workspace build and test
- ✅ **Tauri Build**: Application build verification
- ✅ **Service Testing**: Database connection verification

**Key Features**:
- Comprehensive Rust testing
- Docker service integration
- Build artifact upload
- Service health verification

### **5. LSIF Pipeline** (`lsif.yml`)
**Purpose**: Code analysis and search indexing

**Triggers**:
- Manual trigger
- Push to `main` or `master` (when Rust code changes)
- Weekly schedule (Mondays at 2 AM UTC)

**What it does**:
- ✅ **Code Analysis**: Generates LSIF for code navigation
- ✅ **Metadata Extraction**: Extracts function metadata for search
- ✅ **Qdrant Integration**: Prepares data for code search
- ✅ **Artifact Management**: Uploads and commits analysis files

**Key Features**:
- Code search functionality
- Automated code analysis
- Search index generation
- Weekly updates

## 🚀 **Pipeline Benefits**

### **✅ Development Benefits**
- **Consistent Environment**: Docker services ensure same PostgreSQL/Qdrant versions
- **Fast Feedback**: Multiple parallel workflows for quick results
- **Quality Gates**: Comprehensive checks before merging
- **Integration Testing**: Real database testing in CI

### **✅ Release Benefits**
- **Automated Releases**: Tag-based release automation
- **Installer Distribution**: Automatic MSI/NSIS upload
- **Release Notes**: Auto-generated from commits
- **Artifact Storage**: Long-term storage of releases

### **✅ Quality Benefits**
- **Code Standards**: Enforced formatting and linting
- **Security**: Regular security audits
- **Type Safety**: Frontend type checking
- **Build Verification**: Ensures everything compiles

## 🔧 **Workflow Configuration**

### **Docker Services Integration**
All workflows that need databases use Docker services:
```yaml
services:
  postgresql:
    image: postgres:17.5
    env:
      POSTGRES_DB: mudrock
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
    options: >-
      --health-cmd pg_isready
      --health-interval 10s
      --health-timeout 5s
      --health-retries 5
    ports:
      - 5432:5432
      
  qdrant:
    image: qdrant/qdrant:latest
    options: >-
      --health-cmd "curl -f http://localhost:6333/"
      --health-interval 10s
      --health-timeout 5s
      --health-retries 5
    ports:
      - 6333:6333
      - 6334:6334
```

### **Caching Strategy**
- **Rust**: Cargo cache for faster builds
- **Node.js**: npm cache for frontend dependencies
- **Docker**: Layer caching for services

### **Artifact Management**
- **Build Artifacts**: 7-30 day retention
- **Release Artifacts**: 90 day retention
- **LSIF Artifacts**: 30 day retention

## 🎯 **Usage Guide**

### **For Developers**

#### **Daily Development**
```bash
# 1. Start local Docker services
docker-compose -f docker-compose.dev.yml up -d

# 2. Develop with Tauri
cargo tauri dev

# 3. Push changes (triggers CI)
git push origin feature-branch
```

#### **Quality Checks**
- **Formatting**: `cargo fmt`
- **Linting**: `cargo clippy`
- **Security**: `cargo audit`
- **Frontend**: `npm run lint`

### **For Releases**

#### **Creating a Release**
```bash
# 1. Update version in Cargo.toml
# 2. Commit changes
git add .
git commit -m "Bump version to 1.0.0"

# 3. Create and push tag
git tag v1.0.0
git push origin v1.0.0

# 4. GitHub Actions automatically:
#    - Builds release
#    - Creates GitHub release
#    - Uploads installers
```

### **For Code Search**

#### **LSIF Pipeline**
- **Automatic**: Runs on Rust code changes
- **Manual**: Trigger via GitHub Actions UI
- **Scheduled**: Weekly updates on Mondays

## 📊 **Pipeline Statistics**

### **Workflow Coverage**
- ✅ **Build Testing**: All Rust and frontend code
- ✅ **Integration Testing**: Database and service testing
- ✅ **Quality Assurance**: Code standards and security
- ✅ **Release Management**: Automated releases
- ✅ **Code Analysis**: Search and navigation

### **Service Integration**
- ✅ **PostgreSQL**: All database-dependent tests
- ✅ **Qdrant**: Vector database testing
- ✅ **Docker**: Consistent service environment
- ✅ **Health Checks**: Service availability verification

### **Quality Gates**
- ✅ **Code Formatting**: Enforced Rust and frontend standards
- ✅ **Security**: Regular vulnerability scanning
- ✅ **Type Safety**: Frontend type checking
- ✅ **Build Verification**: Compilation and linking tests

## 🚀 **Next Steps**

### **Immediate**
1. **Test Workflows**: Push changes to trigger workflows
2. **Monitor Results**: Check GitHub Actions for any issues
3. **Optimize**: Adjust caching and parallelization

### **Short Term**
1. **Performance**: Optimize workflow execution time
2. **Coverage**: Add more integration tests
3. **Monitoring**: Add workflow success metrics

### **Long Term**
1. **Deployment**: Add production deployment pipeline
2. **Security**: Add container scanning
3. **Compliance**: Add license and dependency checks

## 🎯 **Conclusion**

MudRock now has a **comprehensive CI/CD pipeline** that provides:

- ✅ **Consistent Development**: Docker services for all developers
- ✅ **Quality Assurance**: Multiple quality gates
- ✅ **Automated Releases**: Tag-based release automation
- ✅ **Code Analysis**: Search and navigation features
- ✅ **Integration Testing**: Real database testing

This pipeline ensures **high-quality, reliable releases** while providing **excellent developer experience** with fast feedback and consistent environments! 🚀 