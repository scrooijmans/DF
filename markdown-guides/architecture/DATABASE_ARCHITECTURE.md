# MudRock Database Architecture

## 🎯 **Overview**

MudRock uses a **hybrid PostgreSQL + Parquet architecture** that combines the best of both worlds: fast metadata queries and efficient large data storage.

## 🏗️ **Current Architecture**

### **Hybrid PostgreSQL + Parquet Approach**

#### **1. PostgreSQL for Metadata & Relationships**
```sql
-- Fast queries on structured data
SELECT w.name, b.name as basin, COUNT(l.id) as log_count
FROM wells w 
JOIN basins b ON w.basin_id = b.id
LEFT JOIN logs l ON w.id = l.well_id
GROUP BY w.id, b.name;
```

#### **2. Parquet Files for Large Data**
- **Already implemented** in your `parquet_files` table!
- Perfect for 10,000+ depth-value pairs
- Columnar storage = fast analytical queries
- Compression (Snappy/GZIP) = 70-90% size reduction
- Time series data ready

#### **3. Local Storage Benefits**
- **Data sovereignty** - your data stays local
- **No auth complexity** - single user
- **Direct file system access** - faster than cloud
- **Offline capability** - work anywhere

## 📊 **Database Schema Analysis**

### **Core Tables Found:**
- `basins`, `wells`, `logs`, `log_types` - Core subsurface data
- `parquet_files` - **Already using Parquet!** 🎉
- `geological_models`, `surfaces`, `surface_points` - 3D modeling
- `zones`, `zone_cutoffs` - Geological zoning
- `fluid_sets`, `markers` - Petrophysical data
- `teams`, `projects`, `project_members` - Collaboration
- `preferences`, `profiles` - User settings
- `data_access_log` - Audit trail

## 🚀 **Implementation Strategy**

### **Option A: Local-First (Recommended for MVP)**
```typescript
// Local PostgreSQL + Local Parquet Files
const config = {
  database: 'postgresql://localhost:5432/mudrock',
  parquetStorage: './data/parquet/',
  compression: 'snappy'
}
```

**Benefits:**
- ✅ No auth needed
- ✅ Full data control
- ✅ Faster file access
- ✅ Works offline
- ✅ Simple deployment

### **Option B: Supabase Hybrid**
```typescript
// Supabase PostgreSQL + Supabase Storage
const config = {
  supabaseUrl: 'https://ptlyfnkyfxwlzfefhbcc.supabase.co',
  supabaseKey: 'your-anon-key',
  storageBucket: 'parquet-files'
}
```

**Benefits:**
- ✅ Cloud backup
- ✅ Multi-user ready
- ✅ Real-time subscriptions
- ❌ Auth complexity
- ❌ Network dependency

## 📈 **Time Series Data Strategy**

### **For LLM Queries:**
1. **Small time series** → PostgreSQL `time_series_data` table
2. **Large time series** → Parquet files with metadata in PostgreSQL
3. **Analytics** → Pre-computed aggregations in PostgreSQL

### **Example Time Series Structure:**
```sql
-- PostgreSQL metadata
CREATE TABLE time_series_logs (
  id UUID PRIMARY KEY,
  name TEXT,
  parquet_file_path TEXT,
  time_range TSRANGE,
  interval TEXT, -- '1 minute', '1 hour'
  columns JSONB
);

-- Parquet file: /data/parquet/well_001_pressure.parquet
-- Columns: timestamp, pressure, temperature, quality_flag
```

## 🎯 **Recommended Implementation**

### **1. Use Local PostgreSQL + Parquet**
- Install PostgreSQL locally
- Use `schema_hybrid.sql` (already created)
- Store parquet files in `./data/parquet/`

### **2. Data Flow:**
```
LAS/SEG-Y Files → Convert to Parquet → Store locally
                ↓
            PostgreSQL metadata
                ↓
            Tauri app queries
```

### **3. Migration Path:**
1. **Start local** (MVP)
2. **Export Supabase data** when needed
3. **Keep Supabase as backup** option

## 📊 **Performance Comparison**

| Data Type | PostgreSQL | Parquet | Hybrid |
|-----------|------------|---------|---------|
| Metadata (1KB) | ⚡ Fast | ❌ Slow | ⚡ Fast |
| Log Data (10MB) | 🐌 Slow | ⚡ Fast | ⚡ Fast |
| Time Series (100MB) | 🐌 Very Slow | ⚡ Fast | ⚡ Fast |
| Analytics | ⚡ Fast | ⚡ Fast | ⚡ Fast |

## 🔧 **Technical Implementation**

### **PostgreSQL Schema**
```sql
-- Core tables for metadata
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

-- Parquet file metadata
CREATE TABLE parquet_files (
    id UUID PRIMARY KEY,
    well_id INTEGER REFERENCES wells(id),
    file_path TEXT NOT NULL,
    file_size BIGINT,
    compression TEXT,
    columns JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### **Parquet File Structure**
```
/data/parquet/
├── well_001/
│   ├── pressure.parquet
│   ├── temperature.parquet
│   └── flow_rate.parquet
├── well_002/
│   ├── pressure.parquet
│   └── temperature.parquet
└── metadata.json
```

## 🎯 **Benefits of Hybrid Approach**

### **✅ Performance Benefits**
- **Fast Metadata Queries**: PostgreSQL for relationships and small data
- **Efficient Large Data**: Parquet for time series and log data
- **Columnar Storage**: Fast analytical queries on large datasets
- **Compression**: 70-90% size reduction

### **✅ Operational Benefits**
- **Local Control**: Data stays on your machine
- **Offline Capable**: Work without internet
- **No Auth Complexity**: Single user setup
- **Direct Access**: Faster than cloud storage

### **✅ Development Benefits**
- **Consistent Environment**: Same setup everywhere
- **Easy Testing**: Local data for development
- **Version Control**: Schema changes tracked in Git
- **Backup Simple**: File system backups

## 🚀 **Next Steps**

### **Immediate**
1. **Install local PostgreSQL**
2. **Use `schema_hybrid.sql`** (already created)
3. **Create parquet file utilities**
4. **Migrate from Supabase** (optional)

### **Short Term**
1. **Implement parquet file management**
2. **Add data conversion utilities**
3. **Create backup/restore procedures**
4. **Optimize query performance**

### **Long Term**
1. **Add data compression options**
2. **Implement data versioning**
3. **Add data validation**
4. **Create data migration tools**

## 🎯 **Conclusion**

The hybrid PostgreSQL + Parquet approach gives you the best of both worlds:

- ✅ **Fast metadata queries** with PostgreSQL
- ✅ **Efficient large data storage** with Parquet
- ✅ **Local data control** for privacy and performance
- ✅ **Offline capability** for field work
- ✅ **Simple setup** for individual users

**Perfect for geoscience applications!** 🛢️ 