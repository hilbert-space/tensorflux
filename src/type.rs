declare! {
    #[doc = "A data type."]
    pub struct Type => TF_DataType,

    Bool => TF_BOOL,
    Half => TF_HALF,
    Float => TF_FLOAT,
    Double => TF_DOUBLE,
    Complex64 => TF_COMPLEX64,
    Complex128 => TF_COMPLEX128,
    Int8 => TF_INT8,
    Int16 => TF_INT16,
    Int32 => TF_INT32,
    Int64 => TF_INT64,
    QInt8 => TF_QINT8,
    QInt16 => TF_QINT16,
    QInt32 => TF_QINT32,
    QUInt8 => TF_QUINT8,
    QUInt16 => TF_QUINT16,
    UInt8 => TF_UINT8,
    UInt16 => TF_UINT16,
    String => TF_STRING,
    BFloat16 => TF_BFLOAT16,
}
