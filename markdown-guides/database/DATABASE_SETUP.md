# MudRock Database Setup Guide

## 🎯 **Overview**

MudRock automatically sets up a complete database with comprehensive well data on every fresh installation. No manual setup required!

## 🚀 **What Happens on Fresh Installation**

### **1. Automatic Database Setup**

When you install and run MudRock for the first time:

1. **PostgreSQL Service Starts** - Embedded PostgreSQL starts automatically
2. **Database Schema Created** - All tables and indexes created automatically
3. **Complete Data Inserted** - 23 wells with realistic Texas coordinates
4. **Ready to Use** - Application immediately shows all wells in the GUI

### **2. What Gets Created**

#### **Database Schema**

- `wells` table with coordinates (X, Y)
- `teams` table for organization
- `projects` table for project management
- `users` table for user management
- Performance indexes on key fields

#### **Sample Data (23 Wells)**

- **5 Basic wells**: Well-001 through Well-005
- **5 Permian Basin wells**: PB-001 through PB-005 (West Texas)
- **5 Eagle Ford wells**: EF-001 through EF-005 (South Texas)
- **5 Barnett Shale wells**: BS-001 through BS-005 (North Texas)
- **3 Custom wells**: My-Well-1, My-Well-2, My-Well-3

#### **Supporting Data**

- **2 teams**: Exploration Team Alpha, Development Team Beta
- **2 projects**: North Basin Exploration, South Basin Development
- **2 users**: Dr. Sarah Johnson, Mike Chen

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

## 🔧 **Technical Implementation**

### **Updated Files**

1. **`data_ingestion.rs`** - Now inserts all 23 wells with realistic coordinates
2. **`database_setup.rs`** - Creates complete schema with all indexes
3. **`db_initializer.rs`** - Orchestrates the complete setup process
4. **`README.md`** - Updated documentation

### **Key Changes**

- **Fixed UUIDs**: Teams, projects, and users now use consistent UUIDs
- **Complete Well Data**: All 23 wells inserted automatically
- **Individual User Friendly**: Wells can be added without UUIDs
- **Realistic Coordinates**: Texas oil field coordinates used

## 🎯 **Benefits**

### **For End Users**

- ✅ **Zero Setup Required** - Just install and run
- ✅ **Immediate Data** - 23 wells ready to view
- ✅ **Realistic Examples** - Texas oil field coordinates
- ✅ **Individual Use** - No UUIDs needed for personal wells

### **For Developers**

- ✅ **Consistent Setup** - Same data every time
- ✅ **Testable** - Easy to reset and test
- ✅ **Extensible** - Easy to add more data
- ✅ **Maintainable** - Clean, focused code

## 🚀 **Usage After Installation**

### **1. Start MudRock**

```bash
cargo tauri dev
```

### **2. View Wells**

- Open the application
- Database grid shows all 23 wells
- Wells are organized by region

### **3. Add Your Own Wells**

```sql
-- Add wells without UUIDs (for individual use)
INSERT INTO wells (name, x, y) VALUES
    ('My-Well-1', -100.123456, 31.456789),
    ('My-Well-2', -100.234567, 31.567890);
```

### **4. View in Application**

- Wells appear immediately in the database grid
- No additional setup required

## 🔍 **Verification**

### **Check Database Content**

```bash
# Count wells
docker exec mudrock-postgresql-dev psql -U postgres -d mudrock -c "SELECT COUNT(*) as wells FROM wells;"

# View all wells
docker exec mudrock-postgresql-dev psql -U postgres -d mudrock -c "SELECT name, x, y FROM wells ORDER BY name;"
```

### **Expected Results**

- ✅ 23 wells in database
- ✅ 2 teams created
- ✅ 2 projects created
- ✅ 2 users created
- ✅ All indexes created
- ✅ Application shows wells in GUI

## 🛠️ **Troubleshooting**

### **If Database is Empty**

```bash
# Reset database
docker exec mudrock-postgresql-dev psql -U postgres -c "DROP DATABASE IF EXISTS mudrock;"
docker exec mudrock-postgresql-dev psql -U postgres -c "CREATE DATABASE mudrock TEMPLATE template0;"

# Restart MudRock application
cargo tauri dev
```

### **If Application Shows No Data**

1. Check PostgreSQL is running: `docker ps`
2. Restart MudRock: `cargo tauri dev`
3. Check console for initialization messages

## 📁 **Backup Files**

The following files are available for manual database management:

| File                           | Purpose                                 |
| ------------------------------ | --------------------------------------- |
| `mudrock-complete-setup.sql`   | Complete database setup script          |
| `mudrock-database-dump.sql`    | PostgreSQL dump of current database     |
| `restore-mudrock-database.bat` | Windows batch script for easy restore   |
| `DATABASE_MANAGEMENT.md`       | Comprehensive database management guide |

## ✅ **Success Indicators**

After fresh installation, you should see:

- ✅ MudRock application starts successfully
- ✅ Console shows "Database initialization complete!"
- ✅ Database grid displays 23 wells
- ✅ All wells have realistic Texas coordinates
- ✅ No manual setup required

## 🎯 **Summary**

MudRock now provides a **complete, ready-to-use experience** on fresh installation:

1. **Automatic Setup** - No manual database configuration
2. **Rich Sample Data** - 23 wells with realistic coordinates
3. **Individual User Friendly** - No complex UUIDs required
4. **Immediate Usability** - Start using the application right away
5. **Extensible** - Easy to add your own wells

**Perfect for both individual users and development teams!** 🛢️
