#
# Autogenerated by Thrift
#
# DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
#  @generated
#
from cpython cimport bool as pbool, int as pint, float as pfloat

cimport folly.iobuf as __iobuf

cimport thrift.py3.builder


cimport test.fixtures.enumstrict.module.types as _test_fixtures_enumstrict_module_types

cdef class MyStruct_Builder(thrift.py3.builder.StructBuilder):
    cdef public _test_fixtures_enumstrict_module_types.MyEnum myEnum
    cdef public _test_fixtures_enumstrict_module_types.MyBigEnum myBigEnum


