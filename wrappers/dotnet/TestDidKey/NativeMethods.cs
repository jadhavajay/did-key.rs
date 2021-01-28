using System;
using System.Collections.Generic;
using System.Diagnostics.CodeAnalysis;
using System.Runtime.InteropServices;
using System.Text;

namespace TestDidKey
{
    [SuppressMessage("Style", "IDE1006:Naming Styles", Justification = "Names must match C callable methods")]
    internal class NativeMethods
    {
#if __IOS__
        internal const string LibraryName = "__Internal";
#else
        internal const string LibraryName = "did_key";
#endif

        #region Keys

        [DllImport(LibraryName, CharSet = CharSet.Auto, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int generate_new(string key_type);

        //[DllImport(LibraryName, CharSet = CharSet.Auto, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        //internal static extern int didkey_convert(ByteBuffer request, out ByteBuffer response, out ExternError error);

        #endregion

        #region Pack

        //[DllImport(LibraryName, CharSet = CharSet.Auto, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        //internal static extern int didcomm_pack(ByteBuffer request, out ByteBuffer response, out ExternError error);

        //[DllImport(LibraryName, CharSet = CharSet.Auto, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        //internal static extern int didcomm_unpack(ByteBuffer request, out ByteBuffer response, out ExternError error);

        #endregion

        #region Sign

        //[DllImport(LibraryName, CharSet = CharSet.Auto, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        //internal static extern int didcomm_sign(ByteBuffer request, out ByteBuffer response, out ExternError error);

        //[DllImport(LibraryName, CharSet = CharSet.Auto, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        //internal static extern int didcomm_verify(ByteBuffer request, out ByteBuffer response, out ExternError error);

        #endregion
    }
}
