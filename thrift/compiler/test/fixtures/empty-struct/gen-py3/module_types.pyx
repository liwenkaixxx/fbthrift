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



cdef class Empty:
    def __init__(
        self
    ):
        self.c_Empty = make_shared[cEmpty]()
        pass

    @staticmethod
    cdef create(shared_ptr[cEmpty] c_Empty):
        inst = <Empty>Empty.__new__(Empty)
        inst.c_Empty = c_Empty
        return inst


    def __richcmp__(self, other, op):
        cdef int cop = op
        if cop not in (2, 3):
            raise TypeError("unorderable types: {}, {}".format(self, other))
        if not (
                isinstance(self, Empty) and
                isinstance(other, Empty)):
            if cop == 2:  # different types are never equal
                return False
            else:         # different types are always notequal
                return True

        cdef cEmpty cself = deref((<Empty>self).c_Empty)
        cdef cEmpty cother = deref((<Empty>other).c_Empty)
        cdef cbool cmp = cself == cother
        if cop == 2:
            return cmp
        return not cmp

    def __hash__(Empty self):
        return hash((
        ))






