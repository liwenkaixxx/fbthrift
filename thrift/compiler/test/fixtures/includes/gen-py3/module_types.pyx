#
# Autogenerated by Thrift
#
# DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
#  @generated
#

from libcpp.memory cimport shared_ptr, make_shared, unique_ptr, make_unique
from libcpp.string cimport string
from libcpp cimport bool as cbool
from libcpp.iterator cimport inserter as cinserter
from cpython cimport bool as pbool
from libc.stdint cimport int8_t, int16_t, int32_t, int64_t
from cython.operator cimport dereference as deref, preincrement as inc
from thrift.lib.py3.thrift_server cimport TException
cimport std_libcpp

from collections.abc import Sequence, Set, Mapping, Iterable
from enum import Enum
cimport py3.module_types
cimport py3.includes_types



cdef class MyStruct:
    def __init__(
        self,
        py3.includes_types.Included MyIncludedField
    ):
        self.c_MyStruct = make_shared[cMyStruct]()
        deref(self.c_MyStruct).MyIncludedField = deref(MyIncludedField.c_Included)
        
    @staticmethod
    cdef create(shared_ptr[cMyStruct] c_MyStruct):
        inst = <MyStruct>MyStruct.__new__(MyStruct)
        inst.c_MyStruct = c_MyStruct
        return inst

    @property
    def MyIncludedField(self):
        cdef shared_ptr[cIncluded] item
        if self.__MyIncludedField is None:
            item = make_shared[cIncluded](deref(self.c_MyStruct).MyIncludedField)
            self.__MyIncludedField = Included.create(item)
        return self.__MyIncludedField
        


    def __richcmp__(self, other, op):
        cdef int cop = op
        if cop not in (2, 3):
            raise TypeError("unorderable types: {}, {}".format(self, other))
        if not (
                isinstance(self, MyStruct) and
                isinstance(other, MyStruct)):
            if cop == 2:  # different types are never equal
                return False
            else:         # different types are always notequal
                return True

        cdef cMyStruct cself = deref((<MyStruct>self).c_MyStruct)
        cdef cMyStruct cother = deref((<MyStruct>other).c_MyStruct)
        cdef cbool cmp = cself == cother
        if cop == 2:
            return cmp
        return not cmp

    def __hash__(MyStruct self):
        return hash((
          self.MyIncludedField,
        ))






