# MudRock Database Management Guide

## 🗄️ **Quick Database Setup**

### **Option 1: Complete Setup Script (Recommended)**

```bash
# Run the complete setup script
docker exec -i mudrock-postgresql-dev psql -U postgres mudrock < mudrock-complete-setup.sql
```

### **Option 2: Restore from Dump**

```bash
# Restore from the database dump
docker exec -i mudrock-postgresql-dev psql -U postgres mudrock < mudrock-database-dump.sql
```

### **Option 3: Windows Batch Script**

```bash
# Run the restore script
restore-mudrock-database.bat
```

## 📊 **What's Included**

### **Database Schema**

- `wells` table with coordinates (X, Y)
- `teams` table for organization
- `projects` table for project management
- `users` table for user management
- Performance indexes on key fields

### **Sample Data**

- **23 wells** with realistic Texas coordinates
- **2 teams** (Exploration Team Alpha, Development Team Beta)
- **2 projects** (North Basin Exploration, South Basin Development)
- **2 users** (Dr. Sarah Johnson, Mike Chen)

### **Well Types**

- **Basic wells**: Well-001 through Well-005
- **Permian Basin wells**: PB-001 through PB-005
- **Eagle Ford wells**: EF-001 through EF-005
- **Barnett Shale wells**: BS-001 through BS-005
- **Custom wells**: My-Well-1, My-Well-2, My-Well-3

## 🚀 **Usage Instructions**

### **For Individual Users**

The database is designed for individual use - you can ignore teams and projects:

```sql
-- Add your own wells (no UUIDs needed)
INSERT INTO wells (name, x, y) VALUES
    ('My-Well-1', -100.123456, 31.456789),
    ('My-Well-2', -100.234567, 31.567890);

-- View your wells
SELECT name, x, y FROM wells ORDER BY name;
```

### **For Teams/Projects**

If you want to use teams and projects:

```sql
-- Add wells with team and project associations
INSERT INTO wells (name, team_id, x, y, project_id) VALUES
    ('Team-Well-1', '550e8400-e29b-41d4-a716-446655440000', -100.123456, 31.456789, '770e8400-e29b-41d4-a716-446655440000');
```

## 🔧 **Database Operations**

### **Create Fresh Database**

```bash
# Drop and recreate database
docker exec mudrock-postgresql-dev psql -U postgres -c "DROP DATABASE IF EXISTS mudrock;"
docker exec mudrock-postgresql-dev psql -U postgres -c "CREATE DATABASE mudrock TEMPLATE template0;"
```

### **Backup Current Database**

```bash
# Create a new dump
docker exec mudrock-postgresql-dev pg_dump -U postgres mudrock > mudrock-backup-$(date +%Y%m%d).sql
```

### **Verify Database Content**

```bash
# Check table counts
docker exec mudrock-postgresql-dev psql -U postgres -d mudrock -c "SELECT 'Wells:' as table_name, COUNT(*) as count FROM wells UNION ALL SELECT 'Teams:', COUNT(*) FROM teams UNION ALL SELECT 'Projects:', COUNT(*) FROM projects UNION ALL SELECT 'Users:', COUNT(*) FROM users;"
```

## 📍 **Coordinate System**

### **Texas Oil Fields**

```
Permian Basin (West Texas): -102 to -103°W, 31 to 32°N
Eagle Ford (South Texas): -98 to -99°W, 28 to 29°N
Barnett Shale (North Texas): -97 to -98°W, 32 to 33°N
```

### **Example Coordinates**

```
Houston: -95.3698, 29.7604
Midland: -102.0774, 31.9974
San Antonio: -98.4936, 29.4241
Dallas: -96.7970, 32.7767
```

## 🛠️ **Troubleshooting**

### **Database Connection Issues**

```bash
# Check if PostgreSQL container is running
docker ps | grep postgres

# Check database exists
docker exec mudrock-postgresql-dev psql -U postgres -l | grep mudrock
```

### **Permission Issues**

```bash
# Connect as postgres user
docker exec -it mudrock-postgresql-dev psql -U postgres mudrock
```

### **Reset Everything**

```bash
# Complete reset
docker exec mudrock-postgresql-dev psql -U postgres -c "DROP DATABASE IF EXISTS mudrock;"
docker exec mudrock-postgresql-dev psql -U postgres -c "CREATE DATABASE mudrock TEMPLATE template0;"
docker exec -i mudrock-postgresql-dev psql -U postgres mudrock < mudrock-complete-setup.sql
```

## 📁 **File Descriptions**

| File                           | Purpose                                      | Size |
| ------------------------------ | -------------------------------------------- | ---- |
| `mudrock-complete-setup.sql`   | Complete database setup with schema and data | ~8KB |
| `mudrock-database-dump.sql`    | PostgreSQL dump of current database          | ~8KB |
| `restore-mudrock-database.bat` | Windows batch script for easy restore        | ~1KB |
| `simple-well-injection.sql`    | Well data only (for individual users)        | ~6KB |

## 🎯 **Quick Commands**

### **Setup Database**

```bash
docker exec -i mudrock-postgresql-dev psql -U postgres mudrock < mudrock-complete-setup.sql
```

### **Check Wells**

```bash
docker exec mudrock-postgresql-dev psql -U postgres -d mudrock -c "SELECT COUNT(*) as wells FROM wells;"
```

### **Add Custom Well**

```bash
docker exec mudrock-postgresql-dev psql -U postgres -d mudrock -c "INSERT INTO wells (name, x, y) VALUES ('My-Custom-Well', -100.123456, 31.456789);"
```

## ✅ **Success Indicators**

After running the setup, you should see:

- ✅ 23 wells in the database
- ✅ 2 teams created
- ✅ 2 projects created
- ✅ 2 users created
- ✅ All indexes created
- ✅ MudRock application can connect and display data

## 🚀 **Next Steps**

1. **Start MudRock application**: `cargo tauri dev`
2. **View wells in the GUI**: Database grid will show all wells
3. **Add your own wells**: Use pgAdmin or the application
4. **Export data**: Use the dump files for backup

Happy well data management! 🛢️
