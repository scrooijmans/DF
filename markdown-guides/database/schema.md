# MudRock Database Schema

## 🎯 **Overview**

This document describes the database schema for MudRock, including tables, relationships, and data types.

## 📊 **Core Tables**

### **Wells Table**
```sql
CREATE TABLE wells (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    team_id UUID,
    x DOUBLE PRECISION,
    y DOUBLE PRECISION,
    project_id UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### **Teams Table**
```sql
CREATE TABLE teams (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### **Projects Table**
```sql
CREATE TABLE projects (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    team_id UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### **Users Table**
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

## 🔗 **Relationships**

### **Well Relationships**
- **Well → Team**: Many-to-one relationship via `team_id`
- **Well → Project**: Many-to-one relationship via `project_id`

### **Project Relationships**
- **Project → Team**: Many-to-one relationship via `team_id`

## 📍 **Coordinate System**

### **Coordinate Fields**
- **x**: Longitude (negative for Western Hemisphere)
- **y**: Latitude (positive for Northern Hemisphere)

### **Texas Oil Fields**
```
Permian Basin (West Texas): -102 to -103°W, 31 to 32°N
Eagle Ford (South Texas): -98 to -99°W, 28 to 29°N
Barnett Shale (North Texas): -97 to -98°W, 32 to 33°N
```

## 🎯 **Data Types**

### **Primary Keys**
- **Wells**: `SERIAL` (auto-incrementing integer)
- **Teams**: `UUID` (fixed UUIDs for consistency)
- **Projects**: `UUID` (fixed UUIDs for consistency)
- **Users**: `UUID` (fixed UUIDs for consistency)

### **Foreign Keys**
- **team_id**: `UUID` (references teams.id)
- **project_id**: `UUID` (references projects.id)

### **Coordinate Types**
- **x, y**: `DOUBLE PRECISION` (64-bit floating point)

### **Timestamps**
- **created_at**: `TIMESTAMP WITH TIME ZONE`
- **updated_at**: `TIMESTAMP WITH TIME ZONE`

## 🔧 **Indexes**

### **Performance Indexes**
```sql
-- Well coordinates for spatial queries
CREATE INDEX idx_wells_coordinates ON wells(x, y);

-- Well names for text search
CREATE INDEX idx_wells_name ON wells(name);

-- Team and project relationships
CREATE INDEX idx_wells_team_id ON wells(team_id);
CREATE INDEX idx_wells_project_id ON wells(project_id);

-- Timestamps for temporal queries
CREATE INDEX idx_wells_created_at ON wells(created_at);
CREATE INDEX idx_wells_updated_at ON wells(updated_at);
```

## 🚀 **Usage Examples**

### **Query All Wells**
```sql
SELECT id, name, x, y, created_at 
FROM wells 
ORDER BY name;
```

### **Query Wells by Team**
```sql
SELECT w.name, w.x, w.y, t.name as team_name
FROM wells w
JOIN teams t ON w.team_id = t.id
WHERE t.name = 'Exploration Team Alpha';
```

### **Query Wells by Region**
```sql
SELECT name, x, y
FROM wells
WHERE x BETWEEN -103 AND -102  -- Permian Basin
  AND y BETWEEN 31 AND 32;
```

### **Add New Well**
```sql
INSERT INTO wells (name, x, y) 
VALUES ('My-Well-1', -100.123456, 31.456789);
```

## 🎯 **Schema Benefits**

### **Flexibility**
- **Individual Use**: Wells can be added without teams/projects
- **Team Use**: Full team and project support
- **Spatial Queries**: Coordinate-based filtering
- **Temporal Queries**: Time-based filtering

### **Performance**
- **Fast Queries**: Optimized indexes for common queries
- **Spatial Search**: Coordinate-based filtering
- **Text Search**: Name-based filtering
- **Relationship Queries**: Efficient joins

### **Data Integrity**
- **Primary Keys**: Unique identification
- **Foreign Keys**: Referential integrity
- **Timestamps**: Audit trail
- **Constraints**: Data validation

## 🚀 **Future Extensions**

### **Potential Additions**
- **Log Data**: Well log measurements
- **Production Data**: Production history
- **Geological Data**: Formation information
- **Equipment Data**: Well equipment details

### **Schema Evolution**
- **Backward Compatibility**: Maintain existing queries
- **Migration Scripts**: Safe schema updates
- **Version Control**: Track schema changes
- **Documentation**: Keep schema docs current

## 🎯 **Conclusion**

The MudRock database schema provides:

- ✅ **Flexible Design**: Supports individual and team use
- ✅ **Performance**: Optimized for common queries
- ✅ **Integrity**: Proper constraints and relationships
- ✅ **Extensibility**: Easy to add new features
- ✅ **Simplicity**: Easy to understand and use

**Perfect for geoscience applications!** 🛢️ 