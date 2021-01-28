using System;

namespace TestDidKey
{
    public static class DIDKey
    {

        public static int GenerateNew(string keyType)
        {
            return NativeMethods.generate_new(keyType);
        }
    }
}
