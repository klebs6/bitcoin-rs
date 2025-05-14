// ---------------- [ File: bitcoin-qt/src/csvmodelwriter.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/csvmodelwriter.h]

/**
  | Export a Qt table model to a CSV file.
  | This is useful for analyzing or post-processing
  | the data in a spreadsheet.
  |
  */
#[Q_OBJECT]
pub struct CSVModelWriter {
    base:     QObject,
    filename: String,
    model:    *const QAbstractItemModel,
    columns:  QList<CsvModelWriterColumn>,
}

pub struct CsvModelWriterColumn
{
    title:  String,
    column: i32,
    role:   i32,
}

lazy_static!{
    pub static ref QtEditRole: i32 = ItemDataRole::EditRole.to_int() as i32;
}

//-------------------------------------------[.cpp/bitcoin/src/qt/csvmodelwriter.cpp]
impl CSVModelWriter {

    pub fn new(
        filename: &String,
        parent:   *mut QObject) -> Self {
    
        todo!();
        /*
        : q_object(parent),
        : filename(_filename),
        : model(nullptr),
        */
    }
    
    pub fn set_model(&mut self, model: *const QAbstractItemModel)  {
        
        todo!();
        /*
            this->model = _model;
        */
    }
    
    pub fn add_column(&mut self, 
        title:  &String,
        column: i32,
        role:   Option<i32>)  {

        let role: i32 = role.unwrap_or(*QtEditRole);
        
        todo!();
        /*
            Column col;
        col.title = title;
        col.column = column;
        col.role = role;

        columns.append(col);
        */
    }
    
    /**
      | Perform export of the model to CSV.
      | 
      | 
      | -----------
      | @return
      | 
      | true on success, false otherwise
      |
      */
    pub fn write(&mut self) -> bool {
        
        todo!();
        /*
            QFile file(filename);
        if(!file.open(QIODevice::WriteOnly | QIODevice::Text))
            return false;
        QTextStream out(&file);

        int numRows = 0;
        if(model)
        {
            numRows = model->rowCount();
        }

        // Header row
        for(int i=0; i<columns.size(); ++i)
        {
            if(i!=0)
            {
                writeSep(out);
            }
            writeValue(out, columns[i].title);
        }
        writeNewline(out);

        // Data rows
        for(int j=0; j<numRows; ++j)
        {
            for(int i=0; i<columns.size(); ++i)
            {
                if(i!=0)
                {
                    writeSep(out);
                }
                QVariant data = model->index(j, columns[i].column).data(columns[i].role);
                writeValue(out, data.toString());
            }
            writeNewline(out);
        }

        file.close();

        return file.error() == QFile::NoError;
        */
    }
}

pub fn write_value(
        f:     &mut QTextStream,
        value: &String)  {
    
    todo!();
        /*
            QString escaped = value;
        escaped.replace('"', "\"\"");
        f << "\"" << escaped << "\"";
        */
}

pub fn write_sep(f: &mut QTextStream)  {
    
    todo!();
        /*
            f << ",";
        */
}

pub fn write_newline(f: &mut QTextStream)  {
    
    todo!();
        /*
            f << "\n";
        */
}
