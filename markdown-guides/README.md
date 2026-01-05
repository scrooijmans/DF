# MudRock Documentation Guide

## 📁 **Directory Structure**

This directory contains comprehensive documentation for the MudRock project, organized by topic and purpose.

## 🗂️ **Organization**

### **📋 Development & Operations**
- **`development/`** - Development setup, CI/CD, deployment guides
- **`database/`** - Database management, setup, and troubleshooting
- **`architecture/`** - System architecture and design decisions

### **🔧 Technical Documentation**
- **`postgres/`** - PostgreSQL-specific documentation
- **`quadrant/`** - Qdrant vector database documentation
- **`rag/`** - RAG (Retrieval-Augmented Generation) pipeline docs

### **📊 Testing & Quality**
- **`testing/`** - Test strategies and quality assurance
- **`schemas/`** - Data schemas and specifications

## 📋 **Development & Operations**

### **Development Guides**
- **`DEVELOPMENT_SETUP.md`** - Complete development environment setup
- **`CI_CD_PIPELINE.md`** - GitHub Actions and automated testing
- **`DEPLOYMENT_STRATEGY.md`** - Docker vs Native deployment strategies

### **Database Documentation**
- **`DATABASE_SETUP.md`** - Fresh installation and database setup
- **`DATABASE_MANAGEMENT.md`** - Database operations and maintenance
- **`DATABASE_DISPLAY_FIX.md`** - UUID conversion fix and troubleshooting

### **Architecture Documentation**
- **`DATABASE_ARCHITECTURE.md`** - PostgreSQL + Parquet hybrid architecture
- **`INITIALIZER_ARCHITECTURE.md`** - Application initialization system

## 🔧 **Technical Documentation**

### **PostgreSQL Documentation**
- **`postgres/`** - PostgreSQL-specific guides and schemas

### **Qdrant Documentation**
- **`quadrant/`** - Vector database setup and configuration
- **`quadrant/cloud/`** - Cloud deployment options
- **`quadrant/superlinked/`** - Superlinked integration

### **RAG Pipeline**
- **`rag/`** - Retrieval-Augmented Generation documentation
- **`rag/pipeline.png`** - RAG pipeline diagram
- **`rag/app.png`** - RAG application architecture

## 📊 **Testing & Quality**

### **Test Strategies**
- **`RUST_TEST_STRATEGY.md`** - Comprehensive Rust testing approach
- **`LSIF_WORKFLOW_SUMMARY.md`** - Code analysis and search indexing

### **Schemas & Specifications**
- **`schema.md`** - General schema documentation
- **`PARQUET_LOG_DATA_SCHEMA.md`** - Parquet data format specifications

## 🎯 **Quick Reference**

### **For New Developers**
1. Start with `development/DEVELOPMENT_SETUP.md`
2. Review `development/CI_CD_PIPELINE.md`
3. Understand `database/DATABASE_SETUP.md`

### **For Database Operations**
1. Use `database/DATABASE_MANAGEMENT.md`
2. Reference `database/DATABASE_DISPLAY_FIX.md`
3. Check `postgres/` for PostgreSQL-specific guides

### **For Architecture Decisions**
1. Review `architecture/DATABASE_ARCHITECTURE.md`
2. Understand `architecture/INITIALIZER_ARCHITECTURE.md`
3. Check `quadrant/` for vector database options

## 📝 **Documentation Standards**

### **File Naming**
- Use `SCREAMING_SNAKE_CASE.md` for main documentation files
- Use `kebab-case.md` for specific guides
- Use descriptive names that indicate content

### **Content Organization**
- Start with overview and purpose
- Include step-by-step instructions
- Provide troubleshooting sections
- Add code examples where relevant

### **Cross-References**
- Link between related documents
- Reference source code files when relevant
- Include version information for dependencies

## 🚀 **Maintenance**

### **Keeping Documentation Current**
- Update when code changes
- Review quarterly for accuracy
- Add new guides as features are added
- Archive outdated documentation

### **Contributing**
- Follow the established structure
- Use consistent formatting
- Include practical examples
- Test all code snippets

---

**Last Updated**: December 2024  
**Version**: 1.0.0 